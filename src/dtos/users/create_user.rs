use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub nickname: Option<String>,
    pub email: String,
    pub birth_date: Option<NaiveDate>,
}

impl CreateUser {
    pub fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            ..Default::default()
        }
    }
    
}