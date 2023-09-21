use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUser {
    username: Option<String>,
    nickname: Option<String>,
    email: Option<String>,
    birth_date: Option<NaiveDate>,
}
