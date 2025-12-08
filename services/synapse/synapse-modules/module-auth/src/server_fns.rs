// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{ChallengeRequest, ChallengeResponse, VerifyChallengeRequest, VerifyChallengeResponse};
use leptos::prelude::*;

#[server(RequestChallengeServer, "/api")]
pub async fn request_challenge_server(
    challenge_request: ChallengeRequest,
) -> Result<ChallengeResponse, ServerFnError> {
    use crate::service::create_challenge;
    use crate::types::AuthDeps;

    let deps: AuthDeps = expect_context();

    let challenge = create_challenge(challenge_request.public_key)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    deps.crypto_repo
        .store_challenge(challenge.clone())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(ChallengeResponse { challenge })
}

#[server(VerifyChallengeServer, "/api")]
pub async fn verify_challenge_server(
    challenge_request: VerifyChallengeRequest,
) -> Result<VerifyChallengeResponse, ServerFnError> {
    use axum::http::header;
    use crate::service::verify_challenge;
    use crate::types::AuthDeps;
    use leptos_axum::ResponseOptions;

    let deps: AuthDeps = expect_context();
    let response = expect_context::<ResponseOptions>();
    let challenge_response = verify_challenge(deps, challenge_request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let token = challenge_response.session.id.clone();
    let ttl_secs = 14_400;
    // TODO add Secure; to cookie for prod
    let cookie_value =
        format!("menexus_session={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={ttl_secs}",);
    response.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&cookie_value).unwrap(),
    );

    Ok(challenge_response)
}
