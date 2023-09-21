use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    username: String,
    nickname: Option<String>,
    email: String,
    birth_date: Option<NaiveDate>,
}
