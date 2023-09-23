use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;

use crate::AppState;
use crate::errors::res_err::ResErr;

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    responses(
        (status = 200, description = "all users", body = Vec<User>)
    )
)]
#[get("")]
pub async fn get_all_users(app_data: web::Data<AppState>) -> Result<impl Responder, ResErr> {
    let users = app_data.db.all().await?;
    Ok(HttpResponse::Ok().json(users))
}
