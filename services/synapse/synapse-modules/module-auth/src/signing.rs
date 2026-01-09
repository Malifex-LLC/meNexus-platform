// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! Client-side event signing utilities.
//!
//! This module provides functions for signing events with the user's private key,
//! enabling federated authentication across Synapses.

#[cfg(feature = "hydrate")]
use k256::ecdsa::{Signature, SigningKey, signature::DigestSigner};
#[cfg(feature = "hydrate")]
use sha2::{Digest, Sha256};

/// Get the signing key from session storage.
/// Returns None if not logged in or key not available.
#[cfg(feature = "hydrate")]
pub fn get_signing_key() -> Option<SigningKey> {
    let window = web_sys::window()?;
    let storage = window.session_storage().ok()??;
    let key_hex = storage.get_item("menexus_signing_key").ok()??;
    
    let key_bytes = hex::decode(key_hex.trim()).ok()?;
    if key_bytes.len() != 32 {
        return None;
    }
    
    SigningKey::from_slice(&key_bytes).ok()
}

/// Check if the user has a signing key available (i.e., is logged in with signing capability).
#[cfg(feature = "hydrate")]
pub fn has_signing_key() -> bool {
    get_signing_key().is_some()
}

/// Sign arbitrary bytes with the user's private key.
/// Returns the hex-encoded signature, or None if signing key is not available.
#[cfg(feature = "hydrate")]
pub fn sign_bytes(payload: &[u8]) -> Option<String> {
    let signing_key = get_signing_key()?;
    let digest = Sha256::new_with_prefix(payload);
    let signature: Signature = signing_key.sign_digest(digest);
    Some(hex::encode(signature.to_bytes()))
}

/// Sign an event payload and return the hex-encoded signature.
/// 
/// This creates a canonical representation of the event for signing.
/// The signature can be verified by any Synapse that has the agent's public key.
#[cfg(feature = "hydrate")]
pub fn sign_event_payload(
    event_type: &str,
    agent: &str,
    content: Option<&str>,
    module_kind: Option<&str>,
    module_slug: Option<&str>,
) -> Option<String> {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct SigningPayload<'a> {
        event_type: &'a str,
        agent: &'a str,
        content: Option<&'a str>,
        module_kind: Option<&'a str>,
        module_slug: Option<&'a str>,
    }
    
    let payload = SigningPayload {
        event_type,
        agent,
        content,
        module_kind,
        module_slug,
    };
    
    let payload_bytes = serde_json::to_vec(&payload).ok()?;
    sign_bytes(&payload_bytes)
}

/// Clear the signing key from session storage (logout).
#[cfg(feature = "hydrate")]
pub fn clear_signing_key() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.session_storage() {
            let _ = storage.remove_item("menexus_signing_key");
        }
    }
}

/// Get the public key (hex-encoded) derived from the stored signing key.
#[cfg(feature = "hydrate")]
pub fn get_public_key() -> Option<String> {
    let signing_key = get_signing_key()?;
    let verifying_key = signing_key.verifying_key();
    let encoded = verifying_key.to_encoded_point(true);
    Some(hex::encode(encoded.as_bytes()))
}
