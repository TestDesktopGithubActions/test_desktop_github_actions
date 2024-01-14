use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub rdc: redis::Client,
    pub page_size: i32,
    pub secret: String,
    pub api_secret: String,
}