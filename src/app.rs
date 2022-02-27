use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::authentication::Authentication;
use crate::error::{InternalError, UserFacingError};

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    pub database: DbPool,
    pub authenticator: Authentication,
    // pub live_users: LiveUsers,
}

impl AppState {
    pub fn new(database: DbPool, authenticator: Authentication /*, live_users: LiveUsers*/) -> Self {
        AppState {
            database,
            authenticator,
            // live_users,
        }
    }

    // fn insert_ignore_members(conn: &MysqlConnection, all_users: Vec<User>) -> Result<(), InternalError> {
    //     use crate::schema::accounts::dsl::*;
    //     use diesel::expression::bound::Bound;
    //     use diesel::sql_types::Text;

    //     // Rust typing system breaks here. Don't ask me why. I think the diesel magic types are
    //     // nested to deep. Dwarves never learn. They always dig deeper.
    //     let values: Vec<diesel::expression::operators::Eq<external_id, Bound<Text, String>>> =
    //         all_users.into_iter().map(|u| external_id.eq(u.id)).collect();

    //     let inserted_rows = diesel::insert_or_ignore_into(members)
    //         .values(values)
    //         .execute(conn)
    //         .map_err(InternalError::DatabaseError)?;

    //     if inserted_rows != 0 {
    //         info!("New user inserted.")
    //         // TODO: with postgres we could detect the inserted rows easily.
    //     }
    //     Ok(())
    // }

    pub fn open_database_connection(&self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, UserFacingError> {
        self.database.get()
            .map_err(|e| UserFacingError::Internal(InternalError::DatabasePoolingError(e)))
    }
}


