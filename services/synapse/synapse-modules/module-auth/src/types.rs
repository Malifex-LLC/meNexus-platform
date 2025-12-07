// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
use synapse_core::domain::auth::Session;
use synapse_core::domain::crypto::CryptoChallenge;

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use synapse_core::ports::auth::SessionRepository;
#[cfg(feature = "ssr")]
use synapse_core::ports::crypto::CryptoRepository;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AuthDeps {
    pub crypto_repo: Arc<dyn CryptoRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChallengeRequest {
    pub public_key: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChallengeResponse {
    pub challenge: CryptoChallenge,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VerifyChallengeRequest {
    pub public_key: String,
    pub challenge: CryptoChallenge,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VerifyChallengeResponse {
    pub session: Session,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogoutRequest {
    pub public_key: String,
    pub signature: String,
}
