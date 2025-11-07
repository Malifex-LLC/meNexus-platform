// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use crate::errors::CoreError;
use crate::ports::events::event_repository::EventRepository;

#[async_trait::async_trait]
pub trait UnitOfWork: Send {
    fn events(&mut self) -> &mut dyn EventRepository;

    async fn commit(self: Box<Self>) -> Result<(), CoreError>;
    async fn rollback(self: Box<Self>) -> Result<(), CoreError>;
}

#[async_trait::async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, CoreError>;
}
