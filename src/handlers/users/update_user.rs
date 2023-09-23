use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::dtos::UpdateUser;
use crate::errors::res_err::ResErr;
use crate::AppState;

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "users",
    responses(
        (status = 200, description = "updated users")
    ),
    params(
        ("id" = Uuid, description = "users id")
    ),
    request_body(content = UpdateUser, description = "update info")
)]
#[put("/{id}")]
pub async fn update_user(
    app_data: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateUser>,
) -> Result<impl Responder, ResErr> {
    let id = path.into_inner();
    let update_user = payload.into_inner();
    let user = app_data.db.update(id, update_user).await?;

    Ok(HttpResponse::Ok().json(user))
}
