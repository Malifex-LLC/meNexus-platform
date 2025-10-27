use axum::Router;

use crate::state::AppState;

pub mod activities;
pub mod auth;
pub mod chats;
pub mod comments;
pub mod federation;
pub mod followers;
pub mod health;
pub mod posts;
pub mod reactions;
pub mod settings;
pub mod synapses;
pub mod users;

pub fn routes() -> Router<AppState> {
    Router::new().merge(health::routes())
}
