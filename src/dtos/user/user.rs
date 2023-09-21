use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(value_type = String, format = Uuid)]
    id: Uuid,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    email: String,
    #[schema(value_type = String, format = Date)]
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_date: Option<NaiveDate>,
}
