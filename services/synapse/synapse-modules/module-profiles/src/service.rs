// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::{errors::ModuleProfilesError, types::ProfilesDeps};
use synapse_application::events::{CreateEventCommand, CreateRemoteEventCommand};
use synapse_core::domain::events::ObjectRef;
use synapse_core::domain::profiles::Profile;

pub async fn get_profile(
    deps: ProfilesDeps,
    agent_public_key: String,
) -> Result<Option<Profile>, ModuleProfilesError> {
    let profile = deps
        .profile_repo
        .get_profile(&agent_public_key)
        .await
        .unwrap();
    if let Some(profile) = profile {
        return Ok(Some(profile));
    }

    let trimmed = agent_public_key.trim().to_string();
    let inner = CreateEventCommand {
        event_type: "profiles:get_profile".into(),
        module_kind: Some("profiles".into()),
        agent: "local-agent".into(), // TODO get synapse public key
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
                return Ok(profile);
            }
        }
    }

    Ok(None)
}

pub async fn fetch_profile_from_peer(
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
