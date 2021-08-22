use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use std::{
    collections::hash_map,
    sync::{mpsc::Sender, Mutex},
};

use crate::{ClassificationRequest, TextResponse, UnclassifiedText};

#[post("/api/v1/texts/unclassified/_next")]
async fn get_unclassified(
    unclassified_texts: web::Data<
        Mutex<hash_map::IntoIter<String, UnclassifiedText>>,
    >,
) -> impl Responder {
    let mut unclassified_texts = unclassified_texts.lock().unwrap();

    match unclassified_texts.next() {
        Some((_key, unclassified_text)) => {
            HttpResponse::Ok().json(TextResponse {
                id: &unclassified_text.id,
                text: &unclassified_text.text,
            })
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/api/v1/texts/classified")]
async fn add_classified(
    classification: web::Json<ClassificationRequest>,
    tx: web::Data<Sender<ClassificationRequest>>,
) -> impl Responder {
    info!(
        "Got classification for text {} as {}",
        classification.id, classification.language
    );

    tx.send(classification.into_inner()).unwrap();

    HttpResponse::NoContent()
}
