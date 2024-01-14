use std::path::PathBuf;

use tauri::Manager;

pub mod db;
pub mod latest_login;
pub mod node;
pub mod user;

pub async fn init_public_sqlite_db(
    public_path: Option<String>,
) -> Result<crate::database::db::PoolSqlite, crate::Error> {
    let pool = crate::database::db::DbConnection::default()
        .pub_migrator()
        .set_uri(public_path)
        .init_database()
        .await
        .map_err(|e| crate::Error::BadRequest(crate::InitDatabaseError::DatabaseError(e).into()))?;
    let conn = pool.get_pool().ok_or(crate::BadRequest::InitDatabase(
        crate::InitDatabaseError::DatabaseError(crate::DatabaseError::GetPublicSqlitePoolFailed),
    ))?;
    let _ = crate::database::latest_login::LatestLogin::insert_if_not_exist(conn)
        .await
        .map_err(|e| crate::Error::BadRequest(crate::InitDatabaseError::DatabaseError(e).into()))?;
    let conn = conn.to_owned();
    crate::database::db::PUBLIC_SQLITE_POOL.get_or_init(|| pool);
    Ok(conn)
}

// type Storage = (PathBuf, PathBuf);

pub fn init_storage(app: &tauri::App) -> Result<PathBuf, crate::Error> {
    let app_dir = app
        .app_handle()
        .path_resolver()
        .app_local_data_dir()
        .unwrap_or_default();
    println!("path: {app_dir:?}");
    let storage = app_dir.join("storage");
    let user_storage = storage.join("users");
    let public_storage = storage.join("public");
    std::fs::create_dir_all(&user_storage)
        .map_err(|_| crate::BadRequest::Storage(crate::IOError::CreateDirAllFailed.into()))?;

    crate::database::db::USER_STORAGE
        .set(user_storage)
        .map_err(|_| {
            crate::Error::BadRequest(
                crate::StorageError::DatabaseError(crate::DatabaseError::SetUserStorageFailed)
                    .into(),
            )
        })?;
    crate::database::db::PUBLIC_STORAGE
        .set(public_storage.clone())
        .map_err(|_| {
            crate::Error::BadRequest(
                crate::StorageError::DatabaseError(crate::DatabaseError::SetPublicStorageFailed)
                    .into(),
            )
        })?;
    std::fs::create_dir_all(&public_storage)
        .map_err(|_| crate::BadRequest::Storage(crate::IOError::CreateDirAllFailed.into()))?;
    Ok(public_storage)
}
