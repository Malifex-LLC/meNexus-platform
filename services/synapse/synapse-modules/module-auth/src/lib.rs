// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use crate::errors::ModuleAuthError;
use async_trait::async_trait;
use axum::{
    Json,
    extract::{FromRef, State},
    http::{HeaderMap, StatusCode, header},
    routing::post,
};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hex;
use k256::{
    EncodedPoint,
    ecdsa::{Signature, VerifyingKey, signature::DigestVerifier, signature::Verifier},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use synapse_core::ports::modules::Module;
use synapse_core::{
    CoreError,
    domain::{auth::Session, crypto::CryptoChallenge, events::Event},
    ports::{
        auth::SessionRepository,
        crypto::{CryptoRepository, errors::CryptoError},
    },
};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub struct AuthModule {
    kind: String,
    version: String,
    crypto_repo: Arc<dyn CryptoRepository>,
    session_repo: Arc<dyn SessionRepository>,
}

impl AuthModule {
    pub fn new(
        session_repo: Arc<dyn SessionRepository>,
        crypto_repo: Arc<dyn CryptoRepository>,
    ) -> Self {
        Self {
            kind: "auth".to_string(),
            version: "1.0.0".to_string(),
            crypto_repo,
            session_repo,
        }
    }
}

#[async_trait]
impl Module for AuthModule {
    fn kind(&self) -> Result<String, CoreError> {
        Ok(self.kind.clone())
    }
    fn version(&self) -> Result<String, CoreError> {
        Ok(self.version.clone())
    }
    async fn handle_event(&self, event: &Event) -> Result<Option<Vec<Event>>, CoreError> {
        match event.event_type.as_str() {
            _ => Ok(None),
        }
    }
}

#[derive(Clone)]
pub struct AuthDeps {
    pub crypto_repo: Arc<dyn CryptoRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
}

pub fn routes<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
    AuthDeps: FromRef<S>,
{
    axum::Router::new()
        .route("/auth/challenges", post(request_challenge_http))
        .route("/auth/login", post(verify_challenge_http))
        .route("/auth/logout", post(handle_logout_http))
}

#[derive(Deserialize)]
struct ChallengeRequest {
    public_key: String,
}

#[derive(Serialize)]
struct ChallengeResponse {
    challenge: CryptoChallenge,
}

#[derive(Deserialize)]
struct VerifyChallengeRequest {
    public_key: String,
    challenge: CryptoChallenge,
    signature: String,
}

#[derive(Deserialize, Serialize)]
struct VerifyChallengeResponse {
    session: Session,
}

#[derive(Deserialize)]
struct LogoutRequest {
    public_key: String,
    signature: String,
}

async fn request_challenge_http(
    State(deps): State<AuthDeps>,
    Json(body): Json<ChallengeRequest>,
) -> Result<(StatusCode, Json<ChallengeResponse>), ModuleAuthError> {
    debug!("request_challenge_http called!");
    let challenge = create_challenge(body.public_key).await.unwrap();
    deps.crypto_repo
        .store_challenge(challenge.clone())
        .await
        .unwrap();
    Ok((StatusCode::CREATED, Json(ChallengeResponse { challenge })))
}

async fn verify_challenge_http(
    State(deps): State<AuthDeps>,
    Json(body): Json<VerifyChallengeRequest>,
) -> Result<(StatusCode, HeaderMap, Json<VerifyChallengeResponse>), ModuleAuthError> {
    debug!("verify_challenge_http called!");
    let challenge = deps
        .crypto_repo
        .get_challenge(body.challenge.id)
        .await
        .unwrap()
        .unwrap();
    let challenge_nonce = challenge.nonce;
    let pk_bytes = hex::decode(body.public_key.clone()).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&pk_bytes).unwrap();
    let sig_bytes = hex::decode(body.signature.clone()).unwrap();
    let signature = Signature::from_slice(&sig_bytes).unwrap();
    let nonce_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(challenge_nonce)
        .unwrap();
    let digest = Sha256::new_with_prefix(&nonce_bytes);
    verifying_key
        .verify_digest(digest, &signature)
        .map_err(|_| ModuleAuthError::BadRequest("signature verification failed".into()))?;
    let expires_at = OffsetDateTime::now_utc() + Duration::from_hours(4);
    let session = Session {
        id: Uuid::new_v4(),
        agent: body.public_key.clone(),
        created_at: OffsetDateTime::now_utc(),
        expires_at,
        revoked: false,
    };
    deps.session_repo.store_session(session.clone()).await;
    let token = session.id.clone();
    let ttl_secs = 14_400;
    // TODO add Secure; to cookie for prod
    let cookie_value =
        format!("menexus_session={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={ttl_secs}",);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&cookie_value).unwrap(),
    );
    Ok((
        StatusCode::CREATED,
        headers,
        Json(VerifyChallengeResponse { session }),
    ))
}

async fn create_challenge(public_key: String) -> Result<CryptoChallenge, ModuleAuthError> {
    let mut nonce_bytes = [0u8; 32];
    getrandom::fill(&mut nonce_bytes);

    let nonce = URL_SAFE_NO_PAD.encode(nonce_bytes);

    let challenge = CryptoChallenge::builder()
        .with_agent(public_key)
        .with_nonce(nonce)
        .build();

    Ok(challenge)
}

async fn handle_logout_http(
    State(deps): State<AuthDeps>,
    Json(body): Json<LogoutRequest>,
) -> Result<(), ModuleAuthError> {
    debug!("handle_logout_http called!");
    Ok(())
}
