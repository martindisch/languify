use actix_cors::Cors;
use actix_web::{
    middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{collections::hash_map, sync::Mutex};

use persistence::UnclassifiedText;

mod persistence;

#[post("/api/v1/texts/unclassified/_next")]
async fn get_unclassified(
    unclassified_texts: web::Data<
        Mutex<hash_map::IntoIter<usize, UnclassifiedText>>,
    >,
) -> impl Responder {
    let mut unclassified_texts = unclassified_texts.lock().unwrap();

    match unclassified_texts.next() {
        Some((id, unclassified_text)) => {
            HttpResponse::Ok().json(TextResponse {
                id,
                text: &unclassified_text.text,
            })
        }
        None => HttpResponse::NotFound().finish(),
    }
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

    info!("Loading unclassified texts from CSV");
    let unclassified_texts = web::Data::new(Mutex::new(
        persistence::load_unclassified("unclassified.csv")
            .expect("Unable to load unclassified texts from CSV")
            .into_iter(),
    ));

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(unclassified_texts.clone())
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
