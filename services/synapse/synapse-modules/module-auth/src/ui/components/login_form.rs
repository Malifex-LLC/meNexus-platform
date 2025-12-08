// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::server_fns::{request_challenge_server, verify_challenge_server};
use crate::types::{ChallengeRequest, ChallengeResponse, VerifyChallengeRequest};
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use hex::decode;
use k256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::DigestSigner};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use sha2::{Digest, Sha256};

#[component]
pub fn LoginForm() -> impl IntoView {
    let (private_key, set_private_key) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (error_msg, set_error_msg) = signal::<Option<String>>(None);
    let (status_msg, set_status_msg) = signal::<Option<String>>(None);

    let on_submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        leptos::logging::log!(
            "Login Form Submitted for private_key: {}!",
            private_key.get()
        );

        let private_key_hex = private_key.get();

        spawn_local(async move {
            let private_key_hex = private_key_hex.trim();

            let private_key_bytes = hex::decode(private_key_hex).expect("invalid hex private key");

            assert_eq!(private_key_bytes.len(), 32, "private key must be 32 bytes");

            let signing_key =
                SigningKey::from_slice(&private_key_bytes).expect("invalid private key bytes");

            let verifying_key = signing_key.verifying_key();

            let encoded = verifying_key.to_encoded_point(true);
            let public_key_hex = hex::encode(encoded.as_bytes());
            leptos::logging::log!("public_key_hex: {}", public_key_hex);
            let crypto_challenge = request_challenge_server(ChallengeRequest {
                public_key: public_key_hex.clone(),
            })
            .await
            .unwrap();

            let nonce = crypto_challenge.challenge.nonce.clone();
            let nonce_bytes = BASE64_URL_SAFE_NO_PAD.decode(nonce).unwrap();
            let digest = Sha256::new_with_prefix(&nonce_bytes);
            let signature: Signature = signing_key.sign_digest(digest);
            let signature_hex = hex::encode(signature.to_bytes());

            let verify_challenge_result = verify_challenge_server(VerifyChallengeRequest {
                public_key: public_key_hex.clone(),
                challenge: crypto_challenge.challenge,
                signature: signature_hex,
            })
            .await;

            match verify_challenge_result {
                Ok(resp) => {
                    // Full page reload to ensure SSR picks up the new session cookie
                    let window = web_sys::window().expect("no window");
                    window.location().set_href("/").expect("failed to redirect");
                }
                Err(err) => {
                    leptos::logging::log!("Login failed: {err}");
                }
            }
        });
    };
    view! {

        <div style="min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #282828; padding: 1rem;">
            <div style="width: 100%; max-width: 28rem;">
                <div style="background-color: #1A1A1A; border-radius: 0.75rem; border: 1px solid #3A3A3A; padding: 2rem; box-shadow: 0 10px 15px rgba(0,0,0,0.5);">

                    // Header
                    <div style="text-align: center; margin-bottom: 2rem;">
                        <h1 style="font-size: 1.875rem; font-weight: bold; color: #EDEDED; margin-bottom: 0.25rem;">
                            "Welcome to"
                        </h1>
                        <h2 style="font-size: 1.875rem; font-weight: bold; color: #FF6B6B; margin-bottom: 1rem;">
                            "meNexus"
                        </h2>
                        <p style="color: #BAA997;">
                            "Login with your private key"
                        </p>
                    </div>

                    // Error message
                    {move || error_msg.get().map(|msg| view! {
                        <div style="background-color: rgba(220, 53, 69, 0.1); border: 1px solid #DC3545; color: #DC3545; border-radius: 0.5rem; padding: 0.75rem; margin-bottom: 1rem; font-size: 0.875rem;">
                            {msg}
                        </div>
                    })}

                    // Status message
                    {move || {
                        if error_msg.get().is_none() {
                            status_msg.get().map(|msg| view! {
                                <div style="background-color: rgba(255, 107, 107, 0.1); border: 1px solid #FF6B6B; color: #FF6B6B; border-radius: 0.5rem; padding: 0.75rem; margin-bottom: 1rem; font-size: 0.875rem;">
                                    {msg}
                                </div>
                            })
                        } else {
                            None
                        }
                    }}

                    // Form
                    <form on:submit=on_submit>
                        <div style="margin-bottom: 1rem;">
                            <label style="display: block; color: #EDEDED; font-weight: 500; margin-bottom: 0.5rem;">
                                "Private Key (hex)"
                            </label>
                            <input
                                type="password"
                                placeholder="64-character hex private key"
                                style="width: 100%; padding: 0.75rem 1rem; background-color: #282828; border: 1px solid #3A3A3A; border-radius: 0.5rem; color: #EDEDED; font-family: monospace; font-size: 0.875rem; box-sizing: border-box;"
                                prop:value=move || private_key.get()
                                on:input=move |ev| {
                                    set_private_key.set(event_target_value(&ev));
                                    set_error_msg.set(None);
                                }
                                prop:disabled=move || is_loading.get()
                            />
                            <p style="font-size: 0.75rem; color: #6A6A6A; margin-top: 0.5rem;">
                                "Your private key stays in your browser and is never sent to the server."
                            </p>
                        </div>

                        <button
                            type="submit"
                            style="width: 100%; padding: 0.75rem; background-color: #FF6B6B; color: #1A1A1A; font-weight: 600; border-radius: 0.5rem; border: none; cursor: pointer; font-size: 1rem;"
                            prop:disabled=move || is_loading.get() || private_key.get().is_empty()
                        >
                            {move || if is_loading.get() { "Authenticating..." } else { "Login" }}
                        </button>
                    </form>

                    // How it works
                    <div style="margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid #3A3A3A;">
                        <p style="font-size: 0.875rem; font-weight: 500; color: #EDEDED; margin-bottom: 0.5rem;">"How it works:"</p>
                        <ol style="font-size: 0.75rem; color: #BAA997; padding-left: 1.25rem; margin: 0;">
                            <li style="margin-bottom: 0.25rem;">"Your public key is derived locally"</li>
                            <li style="margin-bottom: 0.25rem;">"Server sends a random challenge"</li>
                            <li style="margin-bottom: 0.25rem;">"You sign it with your private key"</li>
                            <li>"Server verifies and creates session"</li>
                        </ol>
                    </div>

                    // Register link
                    <div style="margin-top: 1.5rem; text-align: center;">
                        <p style="color: #BAA997;">
                            "Don't have a key? "
                            <A href="/register" attr:style="color: #FF6B6B; text-decoration: none;">
                                "Generate one!"
                            </A>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
