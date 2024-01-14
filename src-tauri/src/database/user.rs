#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct UserInfo {
    pub email: String,
    pub token: String,
}

impl UserInfo {
    #![allow(clippy::too_many_arguments)]
    pub fn new(email: &str, token: &str) -> Self {
        Self {
            email: email.to_string(),
            token: token.to_string(),
        }
    }
}

pub(crate) mod insert {
    use sqlx::{Pool, Sqlite};

    impl super::UserInfo {
        pub async fn insert_user(
            self,
            db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query(
                // "INSERT INTO pk (pk) VALUES (?)"
                "INSERT INTO user ( email, token) 
                VALUES ( $1, $2 ) 
        ON CONFLICT (email) 
        DO UPDATE SET email = EXCLUDED.email, token = EXCLUDED.token",
            )
            .bind(self.email)
            .bind(self.token)
            .execute(db)
            .await
            .map_err(|_| crate::DatabaseError::InsertFailed)?;
            tracing::info!("Query result insert_pk: {:?}", result);
            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::user::UserInfo;

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
            let info = UserInfo::new("gin@techecho.io", "FD34FC32-6F8A-4FC0-AA8E-361AD72C3D3F");
            let results = info.insert_user(pool.get_pool().unwrap()).await.unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}

pub(crate) mod update {
    use sqlx::{Pool, Sqlite};

    impl super::UserInfo {
        pub async fn update_by_email(
            &self,
            _db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let result = sqlx::query("UPDATE user SET email=?,token=?")
                .bind(&self.email)
                .bind(&self.token)
                .execute(_db)
                .await
                .map_err(|_| crate::DatabaseError::UpdateFailed)?;

            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::user::UserInfo;

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

            let info = UserInfo::new("gin@techecho.io", "FD34FC32-6F8A-4FC0-AA8E-361AD72C3D3F");

            info.update_by_email(pool.get_pool().unwrap())
                .await
                .unwrap();
        }
    }
}

pub(crate) mod query {
    use sqlx::{Pool, Sqlite};

    impl super::UserInfo {
        pub async fn get_all(db: &Pool<Sqlite>) -> Result<Vec<Self>, crate::DatabaseError> {
            let results = sqlx::query_as::<_, Self>("SELECT * FROM user")
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

        pub async fn get_user_info_by_email(
            db: &Pool<Sqlite>,
            val: String,
        ) -> Result<Self, crate::DatabaseError> {
            let result = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE email = ?")
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
        use crate::database::user::UserInfo;

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

            let results = UserInfo::get_one(pool.get_pool().unwrap()).await.unwrap();
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

            let results = UserInfo::get_all(pool.get_pool().unwrap()).await.unwrap();
            tracing::info!("res: {results:#?}");
        }
    }
}

pub(crate) mod delete {
    use sqlx::{Pool, Sqlite};

    impl super::UserInfo {
        pub async fn del_one(
            db: &Pool<Sqlite>,
            pk: String,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::DatabaseError> {
            let delete_result = sqlx::query("DELETE FROM user WHERE email=$1")
                .bind(pk)
                .execute(db)
                .await
                .map_err(|_| crate::DatabaseError::DeleteFailed)?;
            tracing::info!("Delete result: {:?}", delete_result);
            Ok(delete_result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::user::UserInfo;

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

            let pk = crate::sys::system_info::SystemInfo::get_sys().to_sha3();
            tracing::info!("[delete_test] pk: {pk}");
            let results = UserInfo::del_one(pool.get_pool().unwrap(), pk)
                .await
                .unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}
