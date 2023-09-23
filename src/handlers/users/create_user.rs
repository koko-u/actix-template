use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::dtos::CreateUser;
use crate::errors::res_err::ResErr;
use crate::AppState;

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "users",
    responses(
        (status = 201, description = "Create new users", body = User)
    )
)]
#[post("")]
pub async fn create_user(
    app_data: web::Data<AppState>,
    payload: web::Json<CreateUser>,
) -> Result<impl Responder, ResErr> {
    let create_user = payload.into_inner();
    let user = app_data.db.insert(create_user).await?;

    Ok(HttpResponse::Created().json(user))
}
