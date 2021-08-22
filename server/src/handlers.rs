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
        Mutex<hash_map::IntoIter<usize, UnclassifiedText>>,
    >,
) -> impl Responder {
    let mut unclassified_texts = unclassified_texts.lock().unwrap();

    match unclassified_texts.next() {
        Some((languify_id, unclassified_text)) => {
            HttpResponse::Ok().json(TextResponse {
                id: languify_id,
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
