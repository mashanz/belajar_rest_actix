mod app_data;
mod api;
use actix_web::{web, App, HttpServer, HttpRequest, Error, get};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_files as fs;

use crate::app_data::AppData;

#[get("/assets/{filename:.*}")]
async fn static_assets(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        // Pre rendering / load to memory
        let tera_data = match tera::Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .app_data(actix_web::web::Data::new(AppData {
                template: tera_data.clone(),
            }))
            .service(static_assets)
            .service(api::base::services())
            .service(api::coba_query_param::services())
            .default_service(web::route().to(api::error_response::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
