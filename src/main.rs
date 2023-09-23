use actix_template::routes::api_route;
use actix_template::ApiDoc;
use actix_template::AppErr;
use actix_template::AppResult;
use actix_template::AppState;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;
use error_stack::ResultExt;
use std::net;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().change_context(AppErr)?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addrs = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Listening on http://{addrs:?}");
    log::info!("Swagger UI http://{addrs:?}/swagger-ui/");

    let app_data = web::Data::new(AppState::new().await?);
    let api_doc = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(api_route(app_data.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api_doc.clone()),
            )
    })
    .bind(addrs)
    .change_context(AppErr)?
    .run()
    .await
    .change_context(AppErr)?;

    Ok(())
}
