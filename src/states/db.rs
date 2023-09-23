use error_stack::ResultExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dtos::CreateUser;
use crate::dtos::UpdateUser;
use crate::dtos::User;
use crate::AppErr;
use crate::AppResult;

pub struct DbState(PgPool);

impl DbState {
    pub async fn init(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::default()
            .connect(database_url)
            .await
            .change_context(AppErr)?;
        Ok(Self(pool))
    }

    pub async fn insert(&self, _create_user: CreateUser) -> AppResult<User> {
        todo!()
    }

    pub async fn update(&self, _id: Uuid, _update_user: UpdateUser) -> AppResult<User> {
        todo!()
    }

    pub async fn all(&self) -> AppResult<Vec<User>> {
        todo!()
    }

    pub async fn get_by_id(&self, _id: Uuid) -> AppResult<Option<User>> {
        todo!()
    }

    pub async fn delete_by_id(&self, _id: Uuid) -> AppResult<()> {
        todo!()
    }
}
