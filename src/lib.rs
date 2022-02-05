#![forbid(unsafe_code)]
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod error;
pub mod auth;
pub mod actions;
pub mod user;
mod keycloak;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::auth::Authenticator;
use crate::error::{InternalError, UserFacingError};
use crate::user::LiveUsers;

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    pub database: DbPool,
    pub authenticator: Authenticator,
    pub live_users: LiveUsers,
}

impl AppState {
    pub fn new(database: DbPool, authenticator: Authenticator, live_users: LiveUsers) -> Self {
        AppState {
            database,
            authenticator,
            live_users,
        }
    }

    pub async fn update(&self) -> Result<(), InternalError> {
        self.authenticator.update().await?;
        self.live_users.update().await
    }
}

pub fn open_database_connection(app: &AppState) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, UserFacingError> {
    app.database.get()
        .map_err(|e| UserFacingError::Internal(InternalError::DatabasePoolingError(e)))
}

