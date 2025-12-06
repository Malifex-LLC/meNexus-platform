// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use crate::errors::ModuleProfilesError;
use async_trait::async_trait;
use automerge::{AutoCommit, ObjId, ReadDoc, ScalarValue, Value, transaction::Transactable};
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::Arc;
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use synapse_core::ports::profiles::profile_repository::ProfileDiscovery;
use synapse_core::{
    CoreError, PersistenceError,
    domain::{
        events::{Event, ObjectRef},
        profiles::Profile,
    },
    ports::{
        modules::Module,
        profiles::profile_repository::{ProfilesDocStore, ProfilesRepository},
    },
};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub struct ProfilesModule {
    kind: String,
    version: String,
    doc_store: Arc<dyn ProfilesDocStore>,
    repo: Arc<dyn ProfilesRepository>,
}

impl ProfilesModule {
    pub fn new(repo: Arc<dyn ProfilesRepository>, doc_store: Arc<dyn ProfilesDocStore>) -> Self {
        Self {
            kind: "profiles".to_string(),
            version: "1.0.0".to_string(),
            repo,
            doc_store,
        }
    }
}

#[async_trait]
impl Module for ProfilesModule {
    fn kind(&self) -> Result<String, CoreError> {
        Ok(self.kind.clone())
    }
    fn version(&self) -> Result<String, CoreError> {
        Ok(self.version.clone())
    }

    async fn handle_event(&self, event: &Event) -> Result<Vec<Event>, CoreError> {
        match event.event_type.as_str() {
            "profiles:get_profile" => {
                let target_pk = match &event.target {
                    Some(ObjectRef::Agent(pk)) => pk.clone(),
                    _ => return Err(CoreError::Validation("target agent required".into())),
                };
                let bytes = self
                    .doc_store
                    .get_doc(&target_pk)
                    .await
                    .map_err(CoreError::from)?;
                let reply = Event::new()
                    .with_event_type("profiles:profile")
                    .with_module_kind("profiles")
                    .with_agent(event.agent.clone())
                    .with_target(ObjectRef::Agent(target_pk))
                    .with_data(bytes.unwrap().clone()) // Option<Vec<u8>>
                    .build();
                Ok(vec![reply])
            }

            "profiles:set_profile" => {
                let owner_pk = match &event.target {
                    Some(ObjectRef::Agent(pk)) => pk.clone(),
                    _ => return Err(CoreError::Validation("target agent required".into())),
                };
                if event.agent != owner_pk {
                    return Err(CoreError::Authorization("owner mismatch".into()));
                }
                let mut doc = if let Some(bytes) = &event.data {
                    ProfileDoc::from_bytes(bytes)
                } else {
                    ProfileDoc::new()
                };
                let new_bytes = {
                    let mut d = doc;
                    d.to_bytes()
                };
                self.doc_store
                    .upsert_doc(&owner_pk, &new_bytes)
                    .await
                    .map_err(CoreError::from)?;
                let reply = Event::new()
                    .with_event_type("profiles:profile")
                    .with_module_kind("profiles")
                    .with_agent(owner_pk.clone())
                    .with_target(ObjectRef::Agent(owner_pk))
                    .with_data(new_bytes)
                    .build();
                Ok(vec![reply])
            }

            _ => Ok(vec![]),
        }
    }
}

#[derive(Clone)]
pub struct ProfilesDeps {
    pub doc_store: Arc<dyn ProfilesDocStore>,
    pub profile_repo: Arc<dyn ProfilesRepository>,
    pub profile_discovery: Arc<dyn ProfileDiscovery>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
}

pub fn routes<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
    ProfilesDeps: axum::extract::FromRef<S>,
{
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/profiles", post(register))
        .route("/profiles/{public_key}", get(get_profile_http))
        .route("/profiles/{public_key}", post(set_profile_http))
        .route(
            "/synapses/{synapse_public_key}/profiles/{public_key}",
            get(get_profile_remote_http),
        )
}

pub struct ProfileDoc {
    inner: AutoCommit,
}

impl ProfileDoc {
    pub fn new() -> Self {
        let mut doc = AutoCommit::new();
        let root = automerge::ROOT;
        doc.put(&root, "handle", "").unwrap();
        doc.put(&root, "display_name", "").unwrap();
        doc.put(&root, "bio", "").unwrap();
        doc.put(&root, "location", "").unwrap();
        doc.put(&root, "avatar_url", "").unwrap();
        Self { inner: doc }
    }
    fn put_string(&mut self, key: &str, value: &str) {
        self.inner.put(&automerge::ROOT, key, value).unwrap();
    }
    pub fn set_display_name(&mut self, name: &str) {
        self.put_string("display_name", name);
    }
    pub fn set_handle(&mut self, handle: &str) {
        self.put_string("handle", handle);
    }
    pub fn set_bio(&mut self, bio: &str) {
        self.put_string("bio", bio);
    }
    pub fn set_location(&mut self, location: &str) {
        self.put_string("location", location);
    }
    pub fn set_avatar_url(&mut self, avatar_url: &str) {
        self.put_string("avatar_url", avatar_url);
    }
    pub fn get_display_name(&self) -> String {
        if let Some((val, _)) = self
            .inner
            .get(&automerge::ROOT, "display_name")
            .ok()
            .flatten()
        {
            if let Value::Scalar(cow) = val {
                if let ScalarValue::Str(s) = cow.as_ref() {
                    return s.to_string();
                }
            }
        }
        String::new()
    }
    pub fn get_handle(&self) -> String {
        if let Some((val, _)) = self.inner.get(&automerge::ROOT, "handle").ok().flatten() {
            if let Value::Scalar(cow) = val {
                if let ScalarValue::Str(s) = cow.as_ref() {
                    return s.to_string();
                }
            }
        }
        String::new()
    }

    pub fn get_bio(&self) -> String {
        if let Some((val, _)) = self.inner.get(&automerge::ROOT, "bio").ok().flatten() {
            if let Value::Scalar(cow) = val {
                if let ScalarValue::Str(s) = cow.as_ref() {
                    return s.to_string();
                }
            }
        }
        String::new()
    }

    pub fn get_location(&self) -> String {
        if let Some((val, _)) = self.inner.get(&automerge::ROOT, "location").ok().flatten() {
            if let Value::Scalar(cow) = val {
                if let ScalarValue::Str(s) = cow.as_ref() {
                    return s.to_string();
                }
            }
        }
        String::new()
    }

    pub fn get_avatar_url(&self) -> String {
        if let Some((val, _)) = self
            .inner
            .get(&automerge::ROOT, "avatar_url")
            .ok()
            .flatten()
        {
            if let Value::Scalar(cow) = val {
                if let ScalarValue::Str(s) = cow.as_ref() {
                    return s.to_string();
                }
            }
        }
        String::new()
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        self.inner.save()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            inner: AutoCommit::load(bytes).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct ProfileRegisterRequest {
    public_key: String,
    display_name: String,
    handle: String,
}

pub async fn register(
    axum::extract::State(deps): axum::extract::State<ProfilesDeps>,
    Json(body): Json<ProfileRegisterRequest>,
) -> Result<(StatusCode, Json<Profile>), ModuleProfilesError> {
    let mut doc = ProfileDoc::new();
    doc.set_display_name(&body.display_name);
    doc.set_handle(&body.handle);
    let mut doc_for_save = doc;
    let doc_bytes = doc_for_save.to_bytes();

    deps.doc_store
        .upsert_doc(&body.public_key, &doc_bytes)
        .await
        .unwrap();
    let cmd = CreateEventCommand {
        event_type: "profiles:set_profile".into(),
        module_kind: Some("profiles".into()),
        agent: body.public_key.clone(),
        target: Some(ObjectRef::Agent(body.public_key.clone())),
        data: Some(doc_bytes),
        ..Default::default()
    };
    deps.create_local_event.execute(cmd).await?;
    deps.profile_discovery
        .announce(&body.public_key)
        .await
        .unwrap();

    let profile = deps
        .profile_repo
        .get_profile(&body.public_key)
        .await
        .unwrap()
        .unwrap();
    Ok((StatusCode::CREATED, Json(profile)))
}

#[derive(Deserialize)]
struct ProfileFetchRequest {
    public_key: String,
}

#[derive(Serialize)]
struct ProfileFetchResult {
    profile: Option<Profile>,
}

async fn get_profile_http(
    axum::extract::State(deps): axum::extract::State<ProfilesDeps>,
    Path(public_key): Path<String>,
) -> Result<(StatusCode, Json<Option<Profile>>), ModuleProfilesError> {
    let profile = deps.profile_repo.get_profile(&public_key).await.unwrap();
    if let Some(profile) = profile {
        return Ok((StatusCode::OK, Json(Some(profile))));
    }

    let trimmed = public_key.trim().to_string();
    let inner = CreateEventCommand {
        event_type: "profiles:get_profile".into(),
        module_kind: Some("profiles".into()),
        agent: "local-agent".into(),
        target: Some(ObjectRef::Agent(trimmed.clone())),
        ..Default::default()
    };

    if let Ok(candidates) = deps.profile_discovery.providers(&trimmed).await {
        for provider_pk in candidates {
            if let Some(_bytes) =
                fetch_profile_from_peer(&deps, &provider_pk, &trimmed, &inner).await?
            {
                // doc is now cached locally; return the freshly loaded profile
                let profile = deps.profile_repo.get_profile(&trimmed).await.unwrap();
                return Ok((StatusCode::OK, Json(profile)));
            }
        }
    }

    Ok((StatusCode::NOT_FOUND, Json(None)))
}

async fn get_profile_remote_http(
    axum::extract::State(deps): axum::extract::State<ProfilesDeps>,
    Path((synapse_public_key, public_key)): Path<(String, String)>,
) -> Result<(StatusCode, Json<Option<Vec<u8>>>), ModuleProfilesError> {
    let trimmed_pk = public_key.trim();
    let inner = CreateEventCommand {
        event_type: "profiles:get_profile".into(),
        module_kind: Some("profiles".into()),
        agent: "local-agent".into(),
        target: Some(ObjectRef::Agent(trimmed_pk.to_string())),
        ..Default::default()
    };

    if let Ok(candidates) = deps.profile_discovery.providers(trimmed_pk).await {
        for peer_pk in candidates {
            match fetch_profile_from_peer(&deps, &peer_pk, trimmed_pk, &inner).await? {
                Some(bytes) => return Ok((StatusCode::OK, Json(Some(bytes)))),
                None => continue,
            }
        }
    }

    // Fallback to the explicit synapse supplied in the URL
    match fetch_profile_from_peer(&deps, &synapse_public_key, trimmed_pk, &inner).await? {
        Some(bytes) => Ok((StatusCode::OK, Json(Some(bytes)))),
        None => Ok((StatusCode::OK, Json(None))),
    }
}

async fn fetch_profile_from_peer(
    deps: &ProfilesDeps,
    peer_public_key: &str,
    profile_public_key: &str,
    inner: &CreateEventCommand,
) -> Result<Option<Vec<u8>>, ModuleProfilesError> {
    let cmd = CreateRemoteEventCommand {
        synapse_public_key: peer_public_key.to_string(),
        event: inner.clone(),
    };

    let events = deps.create_remote_event.execute(cmd).await?;

    if let Some(evt) = events
        .into_iter()
        .find(|e| e.event_type == "profiles:profile")
    {
        if let Some(bytes) = &evt.data {
            deps.doc_store
                .upsert_doc(profile_public_key, bytes)
                .await
                .unwrap();
            deps.profile_discovery
                .announce(profile_public_key)
                .await
                .unwrap();

            return Ok(Some(bytes.clone()));
        }
    }
    Ok(None)
}

#[derive(Deserialize)]
struct SetProfileRequest {
    handle: Option<String>,
    display_name: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    avatar_url: Option<String>,
}

async fn set_profile_http(
    axum::extract::State(deps): axum::extract::State<ProfilesDeps>,
    axum::extract::Path(public_key): axum::extract::Path<String>,
    axum::Json(body): axum::Json<SetProfileRequest>,
) -> Result<(StatusCode, axum::Json<Event>), ModuleProfilesError> {
    let existing_doc = deps.doc_store.get_doc(&public_key).await.unwrap().unwrap();

    let mut doc = ProfileDoc::from_bytes(&existing_doc);

    if let Some(handle) = body.handle.as_deref() {
        doc.set_handle(handle);
    }
    if let Some(display_name) = body.display_name.as_deref() {
        doc.set_display_name(display_name);
    }
    if let Some(bio) = body.bio.as_deref() {
        doc.set_bio(bio);
    }
    if let Some(location) = body.location.as_deref() {
        doc.set_location(location);
    }
    if let Some(avatar_url) = body.avatar_url.as_deref() {
        doc.set_avatar_url(avatar_url);
    }

    let mut doc_for_save = doc;
    let bytes = doc_for_save.to_bytes();
    deps.doc_store
        .upsert_doc(&public_key, &bytes)
        .await
        .unwrap();

    let cmd = CreateEventCommand {
        event_type: "profiles:set_profile".into(),
        module_kind: Some("profiles".into()),
        agent: public_key.clone(), // TODO: set real authenticated agent
        target: Some(ObjectRef::Agent(public_key.clone())),
        data: Some(bytes.clone()),
        ..Default::default()
    };
    let evt = deps.create_local_event.execute(cmd).await?;
    deps.profile_discovery.announce(&public_key).await.unwrap();

    Ok((StatusCode::CREATED, axum::Json(evt)))
}
