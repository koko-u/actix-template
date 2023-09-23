use crate::AppResult;

pub mod db;

pub struct AppState {
    pub db: db::DbState,
}

impl AppState {
    pub async fn new() -> AppResult<Self> {
        let database_url = dotenv_codegen::dotenv!("DATABASE_URL");
        let db = db::DbState::init(database_url).await?;

        Ok(Self { db })
    }
}
