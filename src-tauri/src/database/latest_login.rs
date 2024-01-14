#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct LatestLogin {
    pub email: String,
    pub token: String,
    pub language: String,
    pub count: u32,
    pub expired: i64,
}

impl LatestLogin {
    #![allow(clippy::too_many_arguments)]
    pub fn new(email: &str, token: &str, expired: i64, language: String) -> Self {
        Self {
            email: email.to_string(),
            token: token.to_string(),
            language,
            count: 0,
            expired,
        }
    }
}

pub(crate) mod insert {
    use sqlx::{Pool, Sqlite};

    impl super::LatestLogin {
        pub async fn insert_latest_login(
            self,
            db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query(
                // "INSERT INTO pk (pk) VALUES (?)"
                "INSERT INTO latest_login ( email, token, language, expired) 
                VALUES ( $1, $2, $3, $4 ) 
        ON CONFLICT (email) 
        DO UPDATE SET email = EXCLUDED.email, token = EXCLUDED.token, language = EXCLUDED.language",
            )
            .bind(self.email)
            .bind(self.token)
            .bind(self.language)
            .bind(self.expired)
            .execute(db)
            .await
            .map_err(|_| crate::DatabaseError::InsertFailed)?;

            tracing::info!("Query result insert_pk: {:?}", result);
            Ok(result)
        }

        pub async fn insert_if_not_exist(
            db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query(
                // "INSERT INTO pk (pk) VALUES (?)"
                "INSERT INTO latest_login (email, token, language)
                SELECT '', '', 'en'
                WHERE NOT EXISTS (SELECT 1 FROM latest_login);
                ",
            )
            .execute(db)
            .await
            .map_err(|_| crate::DatabaseError::InsertFailed)?;
            tracing::info!("insert_if_not_exist: {:?}", result);
            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::latest_login::LatestLogin;

        #[tokio::test]
        async fn insert_pk() {
            // tracing_subscriber::fmt()
            //     .pretty()
            //     .with_max_level(tracing::Level::DEBUG)
            //     .with_writer(std::io::stdout)
            //     .init();
            let _ = crate::database::db::DbConnection::default()
                .pub_migrator()
                .set_uri(Some("test".to_string()))
                .init_database()
                .await;

            let pool = crate::database::db::USER_SQLITE_POOL.read().await;
            let pool = pool.as_ref().unwrap();
            let info = LatestLogin::new(
                "gin@techecho.io",
                "FD34FC32-6F8A-4FC0-AA8E-361AD72C3D3F",
                0,
                "en".to_string(),
            );
            let results = info
                .insert_latest_login(pool.get_pool().unwrap())
                .await
                .unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}

pub mod update {
    use sqlx::{Pool, Sqlite};

    impl super::LatestLogin {
        pub async fn update_token(
            &self,
            _db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query("UPDATE latest_login SET email=?,token=?, expired=?")
                .bind(self.email.clone())
                .bind(self.token.clone())
                .bind(self.expired)
                .execute(_db)
                .await
                .map_err(|_| crate::DatabaseError::UpdateFailed)?;

            Ok(result)
        }

        pub async fn update_lang(
            _db: &Pool<Sqlite>,
            language: &str,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query("UPDATE latest_login SET language=?")
                .bind(language)
                .execute(_db)
                .await
                .map_err(|_| crate::DatabaseError::UpdateFailed)?;

            Ok(result)
        }

        pub async fn update_flag(
            _db: &Pool<Sqlite>,
            flag: u32,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query("UPDATE latest_login SET count=?")
                .bind(flag)
                .execute(_db)
                .await
                .map_err(|_| crate::DatabaseError::UpdateFailed)?;

            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::latest_login::LatestLogin;

        #[tokio::test]
        async fn update_token_test() {
            // tracing_subscriber::fmt()
            //     .pretty()
            //     .with_max_level(tracing::Level::DEBUG)
            //     .with_writer(std::io::stdout)
            //     .init();
            let _ = crate::database::db::DbConnection::default()
                .pub_migrator()
                .set_uri(Some("test".to_string()))
                .init_database()
                .await;
            let pool = crate::database::db::USER_SQLITE_POOL.read().await;
            let pool = pool.as_ref().unwrap();

            let info = LatestLogin::new(
                "gin@techecho.io",
                "FD34FC32-6F8A-4FC0-AA8E-361AD72C3D3F",
                0,
                "en".to_string(),
            );

            info.update_token(pool.get_pool().unwrap()).await.unwrap();
        }
    }
}

pub mod query {
    use sqlx::{Pool, Sqlite};

    impl super::LatestLogin {
        pub async fn get_all(db: &Pool<Sqlite>) -> Result<Vec<Self>, crate::DatabaseError> {
            let results = sqlx::query_as::<_, Self>("SELECT * FROM latest_login")
                .fetch_all(db)
                .await
                .map_err(|_| crate::DatabaseError::QueryAllFailed)?;
            Ok(results)
        }

        pub async fn get_one(db: &Pool<Sqlite>) -> Result<Self, crate::DatabaseError> {
            let results = Self::get_all(db)
                .await
                .map_err(|_| crate::DatabaseError::QueryOneFailed)?;
            results
                .first()
                .cloned()
                .ok_or(crate::DatabaseError::QueryOneFailed)
        }

        pub async fn get_token_by_email(
            db: &Pool<Sqlite>,
            val: String,
        ) -> Result<Self, crate::DatabaseError> {
            let result = sqlx::query_as::<_, Self>("SELECT * FROM latest_login WHERE email = ?")
                .bind(val)
                .fetch_one(db)
                .await
                .map_err(|_| crate::DatabaseError::QueryAllFailed)?;
            // println!("Query result: {:?}", result);
            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::latest_login::LatestLogin;

        #[tokio::test]
        async fn get_info() {
            // tracing_subscriber::fmt()
            //     .pretty()
            //     .with_max_level(tracing::Level::DEBUG)
            //     .with_writer(std::io::stdout)
            //     .init();
            let _ = crate::database::db::DbConnection::default()
                .pub_migrator()
                .set_uri(Some("test".to_string()))
                .init_database()
                .await;
            let pool = crate::database::db::USER_SQLITE_POOL.read().await;
            let pool = pool.as_ref().unwrap();

            let results = LatestLogin::get_one(pool.get_pool().unwrap())
                .await
                .unwrap();
            tracing::info!("res: {results:?}");
        }

        #[tokio::test]
        async fn get_all() {
            // tracing_subscriber::fmt()
            //     .pretty()
            //     .with_max_level(tracing::Level::DEBUG)
            //     .with_writer(std::io::stdout)
            //     .init();
            let _ = crate::database::db::DbConnection::default()
                .pub_migrator()
                .set_uri(Some("test".to_string()))
                .init_database()
                .await;
            let pool = crate::database::db::USER_SQLITE_POOL.read().await;
            let pool = pool.as_ref().unwrap();

            let results = LatestLogin::get_all(pool.get_pool().unwrap())
                .await
                .unwrap();
            tracing::info!("res: {results:#?}");
        }
    }
}

pub(crate) mod delete {
    use sqlx::{Pool, Sqlite};

    impl super::LatestLogin {
        pub async fn del_one(
            db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let delete_result = sqlx::query("DELETE FROM latest_login")
                .execute(db)
                .await
                .map_err(|_| crate::DatabaseError::DeleteFailed)?;
            tracing::info!("Delete result: {:?}", delete_result);
            Ok(delete_result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::latest_login::LatestLogin;

        #[tokio::test]
        async fn delete_test() {
            // tracing_subscriber::fmt()
            //     .pretty()
            //     .with_max_level(tracing::Level::DEBUG)
            //     .with_writer(std::io::stdout)
            //     .init();
            let _ = crate::database::db::DbConnection::default()
                .pub_migrator()
                .set_uri(Some("test".to_string()))
                .init_database()
                .await;

            let pool = crate::database::db::USER_SQLITE_POOL.read().await;
            let pool = pool.as_ref().unwrap();

            let results = LatestLogin::del_one(pool.get_pool().unwrap())
                .await
                .unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}
