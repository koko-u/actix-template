use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[utoipa::path(get, path = "/api/health_check")]
#[get("/health_check")]
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().body("Health Check OK")
}
