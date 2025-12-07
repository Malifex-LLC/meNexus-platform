// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

#[cfg(feature = "ssr")]
mod server {
    use crate::errors::ModuleAuthError;
    use crate::service::{create_challenge, verify_challenge};
    use crate::types::{
        AuthDeps, ChallengeRequest, ChallengeResponse, LogoutRequest, VerifyChallengeRequest,
        VerifyChallengeResponse,
    };
    use async_trait::async_trait;
    use axum::{
        Json,
        extract::{FromRef, State},
        http::{HeaderMap, StatusCode, header},
        routing::post,
    };
    use std::sync::Arc;
    use synapse_core::ports::modules::Module;
    use synapse_core::{
        CoreError,
        domain::events::Event,
        ports::{
            auth::SessionRepository,
            crypto::{CryptoRepository, errors::CryptoError},
        },
    };
    use tracing::debug;

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
        async fn handle_event(&self, event: &Event) -> Result<Vec<Event>, CoreError> {
            match event.event_type.as_str() {
                _ => Ok(vec![]),
            }
        }
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
        let challenge_response = verify_challenge(deps, body).await.unwrap();
        let token = challenge_response.session.id.clone();
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
            Json(VerifyChallengeResponse {
                session: challenge_response.session,
            }),
        ))
    }

    async fn handle_logout_http(
        State(deps): State<AuthDeps>,
        Json(body): Json<LogoutRequest>,
    ) -> Result<(), ModuleAuthError> {
        debug!("handle_logout_http called!");
        Ok(())
    }
}

#[cfg(feature = "ssr")]
pub use server::*;
