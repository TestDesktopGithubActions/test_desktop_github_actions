use sqlx::{Pool, Sqlite};

pub static USER_STORAGE: once_cell::sync::OnceCell<std::path::PathBuf> =
    once_cell::sync::OnceCell::new();
pub static PUBLIC_STORAGE: once_cell::sync::OnceCell<std::path::PathBuf> =
    once_cell::sync::OnceCell::new();
// pub(crate) static mut SQLITE_POOL: once_cell::sync::Lazy<once_cell::sync::OnceCell<DbConnection>> =
//     once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

// pub static mut USER_SQLITE_POOL: once_cell::sync::Lazy<once_cell::sync::OnceCell<DbConnection>> =
//     once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);
pub static USER_SQLITE_POOL: once_cell::sync::Lazy<tokio::sync::RwLock<Option<DbConnection>>> =
    once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new(None));

pub static PUBLIC_SQLITE_POOL: once_cell::sync::Lazy<once_cell::sync::OnceCell<DbConnection>> =
    once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

#[derive(Debug, Default)]
pub struct DbConnection {
    pub conn: Option<PoolSqlite>,
    pub uri: String,
    pub migrator: Option<sqlx::migrate::Migrator>,
}

pub type PoolSqlite = Pool<Sqlite>;

impl DbConnection {
    pub fn get_pub_connection<'a>(
    ) -> Result<&'a crate::database::db::DbConnection, crate::DatabaseError> {
        crate::database::db::PUBLIC_SQLITE_POOL
            .get()
            .ok_or(crate::DatabaseError::GetPublicSqliteConnFailed)
    }

    pub fn pub_migrator(mut self) -> Self {
        self.migrator = Some(sqlx::migrate!("./schema/public/migrations"));
        self
    }

    pub fn pri_migrator(mut self) -> Self {
        self.migrator = Some(sqlx::migrate!("./schema/private/migrations"));
        self
    }

    pub async fn run_migrator(&self) -> Result<(), crate::Error> {
        let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(&self.uri).await?;
        self.migrator
            .as_ref()
            .ok_or(crate::Error::BadRequest(
                crate::InitDatabaseError::DatabaseError(crate::DatabaseError::MigratorGetFailed)
                    .into(),
            ))?
            .run(&sqlite_pool)
            .await
            .map_err(|_| {
                crate::Error::BadRequest(
                    crate::InitDatabaseError::DatabaseError(
                        crate::DatabaseError::MigrationRunFailed,
                    )
                    .into(),
                )
            })
    }

    pub fn set_uri(mut self, sqlite_url: Option<String>) -> Self {
        let sqlite_url = sqlite_url.map_or("sqlite://rf.db".to_owned(), |db| db);
        self.uri = sqlite_url;
        self
    }

    pub async fn init_database(mut self) -> Result<DbConnection, crate::DatabaseError> {
        // tracing::info!("sqlite_url: {:?}", self.uri);
        let sqlite_url = &self.uri;
        // tracing::info!("[init_database] sqlite_url: {sqlite_url}");
        async fn _create_database(sqlite_url: &str) -> Result<(), crate::Error> {
            sqlx::Sqlite::create_database(sqlite_url)
                .await
                .map_err(|_| {
                    crate::BadRequest::InitDatabase(crate::InitDatabaseError::DatabaseError(
                        crate::DatabaseError::DatabaseCreateFailed,
                    ))
                    .into()
                })
        }

        use sqlx::migrate::MigrateDatabase as _;
        if !sqlx::Sqlite::database_exists(sqlite_url)
            .await
            .unwrap_or(false)
        {
            _create_database(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseCreateFailed)?;
        };
        let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(sqlite_url)
            .await
            .map_err(|_| crate::DatabaseError::DatabaseConnectFailed)?;
        let sqlite_pool = if self
            .migrator
            .as_ref()
            .ok_or(crate::DatabaseError::MigratorGetFailed)?
            .run(&sqlite_pool)
            .await
            .is_err()
        {
            tracing::error!("migrate filed: remove files & create database again");
            sqlite_pool.close().await;
            // let storage = STORAGE.get().unwrap();
            sqlx::Sqlite::drop_database(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseDropFailed)?;
            _create_database(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseCreateFailed)?;

            let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseConnectFailed)?;
            self.migrator
                .as_ref()
                .ok_or(crate::DatabaseError::MigratorGetFailed)?
                .run(&sqlite_pool)
                .await
                .map_err(|_| crate::DatabaseError::MigrationRunFailed)?;
            sqlite_pool
        } else {
            sqlite_pool
        };
        self.conn = Some(sqlite_pool);
        Ok(self)
    }

    pub fn get_pool(&self) -> Option<&Pool<Sqlite>> {
        self.conn.as_ref()
    }

    pub async fn get_uri(&self) -> String {
        self.uri.clone()
    }
}
