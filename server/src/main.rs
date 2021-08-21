use actix_cors::Cors;
use actix_web::{
    middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[post("/api/v1/texts/unclassified/_next")]
async fn get_unclassified() -> impl Responder {
    HttpResponse::Ok().json(TextResponse {
        id: 1,
        text: "The quick brown fox jumps over the lazy dog",
    })
}

#[post("/api/v1/texts/classified")]
async fn add_classified(
    classification: web::Json<ClassificationRequest>,
) -> impl Responder {
    info!(
        "Got classification for text {} as {}",
        classification.id, classification.language
    );
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .wrap(Cors::permissive())
            .service(get_unclassified)
            .service(add_classified)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[derive(Debug, Serialize)]
struct TextResponse<'a> {
    id: usize,
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct ClassificationRequest {
    id: usize,
    language: String,
}
