#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct NodeInfo {
    #[serde(skip_serializing)]
    pub pk: String,
    pub node_port: u16,
}

impl NodeInfo {
    pub fn new(pk: String, node_port: u16) -> Self {
        Self { pk, node_port }
    }
}

pub(crate) mod insert {
    use sqlx::{Pool, Sqlite};

    impl super::NodeInfo {
        pub async fn insert_node(
            self,
            db: &Pool<Sqlite>,
        ) -> Result<sqlx::sqlite::SqliteQueryResult, crate::Error> {
            let result = sqlx::query(
                // "INSERT INTO pk (pk) VALUES (?)"
                "INSERT INTO node ( pk, node_port) 
                VALUES ( $1, $2 ) 
        ON CONFLICT (pk) 
        DO UPDATE SET pk = EXCLUDED.pk, node_port = EXCLUDED.node_port",
            )
            .bind(self.pk)
            .bind(self.node_port)
            .execute(db)
            .await?;
            tracing::info!("Query result insert_node: {:?}", result);
            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::node::NodeInfo;

        #[tokio::test]
        async fn insert_node() {
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
            let info = NodeInfo::new(pk, 12415);
            let results = info.insert_node(pool.get_pool().unwrap()).await.unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}

pub(crate) mod update {
    use sqlx::{Pool, Sqlite};

    impl super::NodeInfo {
        pub async fn by_pk_update(
            &self,
            _db: &Pool<Sqlite>,
        ) -> anyhow::Result<sqlx::sqlite::SqliteQueryResult> {
            let pk = self.pk.clone();
            tracing::info!("pk: {:?}", pk);
            tracing::info!("self: {:?}", self);
            let result = sqlx::query("UPDATE node SET node_port=? WHERE pk=?")
                .bind(self.node_port)
                .bind(pk)
                .execute(_db)
                .await?;

            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::node::NodeInfo;

        #[tokio::test]
        async fn update_node_port_test() {
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
            let info = NodeInfo::new(pk, 23456);

            let res = NodeInfo {
                pk: info.pk,
                node_port: 43534,
            }
            .by_pk_update(pool.get_pool().unwrap())
            .await
            .unwrap();

            tracing::info!("res: {:?}", res);
        }
    }
}

pub(crate) mod query {
    use sqlx::{Pool, Sqlite};

    impl super::NodeInfo {
        pub async fn get_all(db: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
            let results = sqlx::query_as::<_, Self>("SELECT * FROM node")
                .fetch_all(db)
                .await?;
            Ok(results)
        }

        pub async fn get_one(db: &Pool<Sqlite>) -> Result<Self, crate::Error> {
            let results = Self::get_all(db).await?;
            results
                .first()
                .cloned()
                .ok_or(crate::DatabaseError::QueryOneFailed.into())
        }

        pub async fn get_node_by_pk(db: &Pool<Sqlite>, pk: &str) -> Result<Self, crate::Error> {
            let result = sqlx::query_as::<_, Self>("SELECT * FROM node WHERE pk = ?")
                .bind(pk)
                .fetch_one(db)
                .await?;
            // println!("Query result: {:?}", result);
            Ok(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::node::NodeInfo;

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

            let results = NodeInfo::get_one(pool.get_pool().unwrap()).await.unwrap();
            println!("res: {results:?}");
            let node = NodeInfo::new(
                "0ee5aa4a1a4b139da7ed1c4099e42077ee133f7ed0255ee12b259b9a59df58b6".to_string(),
                43534,
            );
            assert_eq!(node.pk, results.pk);
            assert_eq!(node.node_port, results.node_port)
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

            let results = NodeInfo::get_all(pool.get_pool().unwrap()).await.unwrap();
            println!("res: {results:#?}");
        }

        #[tokio::test]
        async fn get_node_by_pk() {
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

            let results = NodeInfo::get_node_by_pk(
                pool.get_pool().unwrap(),
                "0ee5aa4a1a4b139da7ed1c4099e42077ee133f7ed0255ee12b259b9a59df58b6",
            )
            .await
            .unwrap();
            println!("res: {results:#?}");
        }
    }
}

pub(crate) mod delete {
    use sqlx::{Pool, Sqlite};

    impl super::NodeInfo {
        pub async fn del_one(
            db: &Pool<Sqlite>,
            pk: String,
        ) -> anyhow::Result<sqlx::sqlite::SqliteQueryResult> {
            let delete_result = sqlx::query("DELETE FROM node WHERE pk=$1")
                .bind(pk)
                .execute(db)
                .await?;
            tracing::info!("Delete result: {:?}", delete_result);
            Ok(delete_result)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::database::node::NodeInfo;

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
            let results = NodeInfo::del_one(pool.get_pool().unwrap(), pk)
                .await
                .unwrap();
            tracing::info!("res: {results:?}");
        }
    }
}
