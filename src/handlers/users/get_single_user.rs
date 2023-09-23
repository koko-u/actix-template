use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::errors::res_err::ResErr;
use crate::AppState;

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    responses(
        (status = 200, description = "the users with id", body = User),
        (status = 404, description = "no users found"),
    ),
    params(
        ("id" = Uuid, description = "users id")
    )
)]
#[get("/{id}")]
pub async fn get_single_user(
    app_data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ResErr> {
    let id = path.into_inner();
    let user = app_data.db.get_by_id(id).await?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
