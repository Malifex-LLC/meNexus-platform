// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::ModuleAuthError;
use crate::types::{AuthDeps, VerifyChallengeRequest, VerifyChallengeResponse};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hex;
use k256::ecdsa::{Signature, VerifyingKey, signature::DigestVerifier};
use sha2::{Digest, Sha256};
use std::time::Duration;
use synapse_core::domain::{auth::Session, crypto::CryptoChallenge};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub async fn create_challenge(public_key: String) -> Result<CryptoChallenge, ModuleAuthError> {
    let mut nonce_bytes = [0u8; 32];
    getrandom::fill(&mut nonce_bytes).map_err(|e| {
        ModuleAuthError::Internal(format!("Failed to generate random bytes: {}", e))
    })?;

    let nonce = URL_SAFE_NO_PAD.encode(nonce_bytes);

    let challenge = CryptoChallenge::builder()
        .with_agent(public_key)
        .with_nonce(nonce)
        .build();

    Ok(challenge)
}

pub async fn verify_challenge(
    deps: AuthDeps,
    challenge_request: VerifyChallengeRequest,
) -> Result<VerifyChallengeResponse, ModuleAuthError> {
    debug!("verify_challenge called from service!");
    let challenge = deps
        .crypto_repo
        .get_challenge(challenge_request.challenge.id)
        .await
        .unwrap()
        .unwrap();
    let challenge_nonce = challenge.nonce;
    let pk_bytes = hex::decode(challenge_request.public_key.clone()).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&pk_bytes).unwrap();
    let sig_bytes = hex::decode(challenge_request.signature.clone()).unwrap();
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
        agent: challenge_request.public_key.clone(),
        created_at: OffsetDateTime::now_utc(),
        expires_at,
        revoked: false,
    };
    deps.session_repo.store_session(session.clone()).await;
    Ok(VerifyChallengeResponse { session })
}
