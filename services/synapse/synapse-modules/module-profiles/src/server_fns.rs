// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use synapse_core::domain::profiles::Profile;

#[cfg(feature = "ssr")]
use crate::types::ProfilesDeps;
#[cfg(feature = "ssr")]
use module_auth::types::AuthDeps;

#[server(GetSessionUserProfile, "/api")]
pub async fn get_session_user_profile() -> Result<Option<Profile>, ServerFnError> {
    use axum::http::header;
    use leptos_axum::extract;
    use tracing::debug;

    let headers: axum::http::HeaderMap = extract().await?;
    
    let cookie_header = headers.get(header::COOKIE);
    debug!("Cookie header: {:?}", cookie_header);
    
    let session_id = cookie_header
        .and_then(|value| value.to_str().ok())
        .and_then(|cookies| {
            debug!("Cookies string: {}", cookies);
            cookies
                .split(';')
                .find_map(|cookie| {
                    let trimmed = cookie.trim();
                    debug!("Checking cookie: '{}'", trimmed);
                    trimmed.strip_prefix("menexus_session=")
                })
        });

    debug!("Extracted session_id: {:?}", session_id);

    let Some(session_id) = session_id else {
        debug!("No session_id found, returning None");
        return Ok(None);
    };

    let auth_deps: AuthDeps = expect_context();
    let profile_deps: ProfilesDeps = expect_context();

    debug!("Looking up session with id: {}", session_id);
    let session = auth_deps
        .session_repo
        .get_session(session_id.parse()?)
        .await
        .map_err(|e| ServerFnError::new(format!("Session lookup failed: {}", e)))?;

    debug!("Found session for agent: {}", session.agent);
    let profile = profile_deps
        .profile_repo
        .get_profile(&session.agent)
        .await
        .map_err(|e| ServerFnError::new(format!("Profile lookup failed: {}", e)))?;
    
    debug!("Profile result: {:?}", profile);
    Ok(profile)
}
