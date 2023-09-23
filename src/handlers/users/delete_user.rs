use actix_web::delete;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use uuid::Uuid;

use crate::errors::res_err::ResErr;
use crate::AppState;

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, description = "users id")
    )
)]
#[delete("/{id}")]
pub async fn delete_user(
    app_data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ResErr> {
    let id = path.into_inner();
    app_data.db.delete_by_id(id).await?;

    Ok(HttpResponse::Ok().body(format!("Delete users {id}")))
}
