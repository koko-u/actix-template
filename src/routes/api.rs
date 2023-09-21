use actix_web::middleware;
use actix_web::web;

use crate::handlers::*;

pub fn api_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(middleware::NormalizePath::trim())
            .service(health_check_handler)
            .service(
                web::scope("/users")
                    .service(get_all_users)
                    .service(get_single_user)
                    .service(create_user)
                    .service(update_user)
                    .service(delete_user),
            ),
    );
}
