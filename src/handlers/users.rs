use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "all users", body = Vec<User>)
    )
)]
#[get("")]
pub async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().body("Get all users")
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "the user with id", body = User),
        (status = 404, description = "no users found"),
    ),
    params(
        ("id" = i32, description = "user id")
    )
)]
#[get("/{id}")]
pub async fn get_single_user(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Get single user {id}"))
}

#[utoipa::path(
    post,
    path = "/api/users",
    responses(
        (status = 201, description = "Create new user", body = User)
    )
)]
#[post("")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Get all users")
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "update user")
    ),
    params(
        ("id" = i32, description = "user id")
    )
)]
#[put("/{id}")]
pub async fn update_user(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Update user {id}"))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, description = "user id")
    )
)]
#[delete("/{id}")]
pub async fn delete_user(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Delete user {id}"))
}
