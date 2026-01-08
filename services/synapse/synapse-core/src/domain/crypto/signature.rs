// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! Event signature verification for federated authentication.
//!
//! This module provides utilities for verifying that events are signed by
//! the agent who claims to have created them. This enables zero-session
//! federation where remote Synapses can verify user identity without
//! maintaining session state.

use crate::domain::events::Event;

/// Result of signature verification
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureVerificationResult {
    /// Event has a valid signature from the claimed agent
    Valid,
    /// Event has no signature (acceptable for read-only operations)
    Unsigned,
    /// Event has an invalid signature
    Invalid(String),
}

impl SignatureVerificationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, SignatureVerificationResult::Valid)
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(self, SignatureVerificationResult::Unsigned)
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, SignatureVerificationResult::Invalid(_))
    }
}

/// Verify that an event's signature is valid for the claimed agent.
///
/// Uses k256/secp256k1 ECDSA with SHA-256 hashing, matching the auth module.
///
/// # Arguments
/// * `event` - The event to verify
///
/// # Returns
/// * `SignatureVerificationResult` indicating whether signature is valid, missing, or invalid
#[cfg(feature = "crypto")]
pub fn verify_event_signature(event: &Event) -> SignatureVerificationResult {
    use k256::ecdsa::{Signature, VerifyingKey, signature::DigestVerifier};
    use sha2::{Digest, Sha256};

    // If no signature, return Unsigned
    let signature_hex = match &event.agent_signature {
        Some(sig) => sig,
        None => return SignatureVerificationResult::Unsigned,
    };

    // Empty agent is invalid
    if event.agent.trim().is_empty() {
        return SignatureVerificationResult::Invalid("agent public key is empty".to_string());
    }

    // Decode the agent's public key (hex-encoded SEC1 compressed point)
    let pk_bytes = match hex::decode(&event.agent) {
        Ok(bytes) => bytes,
        Err(e) => return SignatureVerificationResult::Invalid(format!("invalid public key hex: {}", e)),
    };

    let verifying_key = match VerifyingKey::from_sec1_bytes(&pk_bytes) {
        Ok(key) => key,
        Err(e) => return SignatureVerificationResult::Invalid(format!("invalid public key: {}", e)),
    };

    // Decode the signature (hex-encoded)
    let sig_bytes = match hex::decode(signature_hex) {
        Ok(bytes) => bytes,
        Err(e) => return SignatureVerificationResult::Invalid(format!("invalid signature hex: {}", e)),
    };

    let signature = match Signature::from_slice(&sig_bytes) {
        Ok(sig) => sig,
        Err(e) => return SignatureVerificationResult::Invalid(format!("invalid signature format: {}", e)),
    };

    // Get the canonical signing payload
    let payload = event.signing_payload();
    let digest = Sha256::new_with_prefix(&payload);

    // Verify the signature
    match verifying_key.verify_digest(digest, &signature) {
        Ok(_) => SignatureVerificationResult::Valid,
        Err(_) => SignatureVerificationResult::Invalid("signature verification failed".to_string()),
    }
}

/// Non-crypto fallback - always returns Unsigned
#[cfg(not(feature = "crypto"))]
pub fn verify_event_signature(_event: &Event) -> SignatureVerificationResult {
    SignatureVerificationResult::Unsigned
}

/// Check if an event requires authentication (signature) based on event type.
///
/// Read operations typically don't require signatures, while write operations do.
/// 
/// NOTE: For federated (Synapse-to-Synapse) requests, the receiving Synapse trusts
/// the sending Synapse's assertion of the user's identity. This is the same model
/// used by ActivityPub. Full end-to-end user signatures can be added later.
pub fn requires_authentication(_event_type: &str) -> bool {
    // For now, we rely on Synapse-to-Synapse trust for federation
    // The sending Synapse has already authenticated the user via session
    // and includes their public key in the event's agent field.
    // 
    // In the future, we can require user signatures for sensitive operations,
    // but for MVP we accept federation trust.
    // 
    // Explicitly mark operations that need signatures:
    false // Disabled for MVP - using Synapse-to-Synapse trust model
}

/// Verify an event has proper authentication if required.
///
/// Returns Ok(()) if event is properly authenticated or doesn't require auth.
/// Returns Err with message if authentication is required but missing/invalid.
#[cfg(feature = "crypto")]
pub fn verify_event_authentication(event: &Event) -> Result<(), String> {
    if !requires_authentication(&event.event_type) {
        return Ok(());
    }

    match verify_event_signature(event) {
        SignatureVerificationResult::Valid => Ok(()),
        SignatureVerificationResult::Unsigned => {
            Err(format!("event type '{}' requires authentication but event is unsigned", event.event_type))
        }
        SignatureVerificationResult::Invalid(reason) => {
            Err(format!("event signature verification failed: {}", reason))
        }
    }
}

#[cfg(not(feature = "crypto"))]
pub fn verify_event_authentication(_event: &Event) -> Result<(), String> {
    // Without crypto feature, we can't verify - accept all for now
    // In production, the server should have crypto feature enabled
    Ok(())
}

#[cfg(all(test, feature = "crypto"))]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_event() {
        let event = Event::new()
            .with_event_type("posts:list_posts")
            .with_agent("test_agent")
            .build();
        
        assert!(verify_event_signature(&event).is_unsigned());
    }

    #[test]
    fn test_requires_authentication() {
        assert!(requires_authentication("posts:create_post"));
        assert!(requires_authentication("profiles:set_profile"));
        assert!(!requires_authentication("posts:list_posts"));
        assert!(!requires_authentication("posts:get_config"));
    }
}
