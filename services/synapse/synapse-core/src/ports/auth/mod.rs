// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use crate::PersistenceError;
use crate::domain::auth::Session;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn store_session(&self, session: Session) -> Result<Session, PersistenceError>;
    async fn get_session(&self, id: Uuid) -> Result<Session, PersistenceError>;
    async fn revoke_session(&self, id: Uuid) -> Result<Session, PersistenceError>;
    async fn delete_session(&self, id: Uuid) -> Result<(), PersistenceError>;
}
