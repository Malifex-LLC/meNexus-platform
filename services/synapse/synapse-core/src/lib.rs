// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod config;
pub mod domain;
pub mod errors;
pub mod ports;

pub use domain::events::CoreEvent;
pub use errors::CoreError;
pub use ports::federation::errors::TransportError;
pub use ports::persistence::errors::PersistenceError;
