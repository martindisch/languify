use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[post("/api/v1/texts/unclassified/_next")]
async fn get_unclassified() -> impl Responder {
    println!("Serving response");
    HttpResponse::Ok().json(TextResponse {
        id: 1,
        text: "The quick brown fox jumps over the lazy dog",
    })
}

#[post("/api/v1/texts/classified")]
async fn add_classified(
    classification: web::Json<ClassificationRequest>,
) -> impl Responder {
    println!(
        "Got classification for text {} as {}",
        classification.id, classification.language
    );
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
