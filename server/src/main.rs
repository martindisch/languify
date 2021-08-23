use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use log::LevelFilter;
use std::{
    sync::{
        mpsc::{self},
        Mutex,
    },
    thread,
};

use languify_server::{handlers, persistence};

const UNCLASSIFIED_PATH: &str = "unclassified.csv";
const CLASSIFIED_PATH: &str = "classified.csv";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Loading unclassified texts from CSV");
    let unclassified_texts =
        persistence::load_unclassified(UNCLASSIFIED_PATH, CLASSIFIED_PATH)
            .expect("Unable to load unclassified texts from CSV");

    info!("Spawning classified texts writer");
    let (tx, rx) = mpsc::channel();
    let unclassified_texts_copy = unclassified_texts.clone();
    thread::spawn(|| {
        persistence::classified_writer(
            CLASSIFIED_PATH,
            persistence::get_headers(UNCLASSIFIED_PATH)?,
            unclassified_texts_copy,
            rx,
        )
    });

    let unclassified_texts =
        web::Data::new(Mutex::new(unclassified_texts.into_iter()));

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(unclassified_texts.clone())
            .data(tx.clone())
            .wrap(logger)
            .wrap(Cors::permissive())
            .service(handlers::get_unclassified)
            .service(handlers::add_classified)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
