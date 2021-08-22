//! API endpoint handlers.

use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use std::{
    collections::hash_map,
    sync::{mpsc::Sender, Mutex},
};

use crate::{
    ClassifiedTextRequest, UnclassifiedText, UnclassifiedTextResponse,
};

/// Returns the next unclassified text.
#[post("/api/v1/texts/unclassified/_next")]
async fn get_unclassified(
    unclassified_texts: web::Data<
        Mutex<hash_map::IntoIter<String, UnclassifiedText>>,
    >,
) -> impl Responder {
    let mut unclassified_texts = unclassified_texts.lock().unwrap();

    match unclassified_texts.next() {
        Some((_, unclassified_text)) => {
            HttpResponse::Ok().json(UnclassifiedTextResponse {
                id: &unclassified_text.id,
                text: &unclassified_text.text,
            })
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Accepts a classified text.
#[post("/api/v1/texts/classified")]
async fn add_classified(
    classified_text: web::Json<ClassifiedTextRequest>,
    tx: web::Data<Sender<ClassifiedTextRequest>>,
) -> impl Responder {
    info!(
        "Got classification for text {} as {}",
        classified_text.id, classified_text.language
    );

    tx.send(classified_text.into_inner()).unwrap();

    HttpResponse::NoContent()
}
