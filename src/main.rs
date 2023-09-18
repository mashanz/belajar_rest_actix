mod api;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::base::services())
            .service(api::coba_query_param::services())
            .default_service(web::route().to(api::error_response::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
