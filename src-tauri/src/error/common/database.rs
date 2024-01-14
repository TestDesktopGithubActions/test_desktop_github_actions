#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database create failed")]
    DatabaseCreateFailed,
    #[error("Database drop failed")]
    DatabaseDropFailed,
    #[error("Database connect failed")]
    DatabaseConnectFailed,

    // conn
    #[error("Get public database connection failed")]
    GetPublicSqliteConnFailed,
    #[error("Get user database connection failed")]
    GetUserSqliteConnFailed,

    // pool
    #[error("Get public database pool failed")]
    GetPublicSqlitePoolFailed,
    #[error("Get user database pool failed")]
    GetUserSqlitePoolFailed,

    // storage
    #[error("Set public storage failed")]
    SetPublicStorageFailed,
    #[error("Get public storage failed")]
    GetPublicStorageFailed,
    #[error("Set user storage failed")]
    SetUserStorageFailed,
    #[error("Get user storage failed")]
    GetUserStorageFailed,

    // action
    #[error("Query data failed")]
    QueryAllFailed,
    #[error("Query one piece of data failed")]
    QueryOneFailed,
    #[error("Insert failed")]
    InsertFailed,
    #[error("Update failed")]
    UpdateFailed,
    #[error("Delete failed")]
    DeleteFailed,

    // migrate
    #[error("Get migrator failed")]
    MigratorGetFailed,
    #[error("Run migration failed")]
    MigrationRunFailed,
}

impl DatabaseError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            DatabaseError::DatabaseCreateFailed => 6000,
            DatabaseError::DatabaseDropFailed => 6001,
            DatabaseError::DatabaseConnectFailed => 6002,
            DatabaseError::GetPublicSqliteConnFailed => 6003,
            DatabaseError::GetUserSqliteConnFailed => 6004,
            DatabaseError::GetPublicSqlitePoolFailed => 6005,
            DatabaseError::GetUserSqlitePoolFailed => 6006,
            DatabaseError::SetPublicStorageFailed => 6007,
            DatabaseError::GetPublicStorageFailed => 6008,
            DatabaseError::SetUserStorageFailed => 6009,
            DatabaseError::GetUserStorageFailed => 6010,
            DatabaseError::QueryAllFailed => 6011,
            DatabaseError::QueryOneFailed => 6012,
            DatabaseError::InsertFailed => 6013,
            DatabaseError::UpdateFailed => 6014,
            DatabaseError::DeleteFailed => 6015,
            DatabaseError::MigratorGetFailed => 6016,
            DatabaseError::MigrationRunFailed => 6017,
        }
    }
}
