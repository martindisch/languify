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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Loading unclassified texts from CSV");
    let (headers, unclassified_texts) =
        persistence::load_unclassified("unclassified.csv", "classified.csv")
            .expect("Unable to load unclassified texts from CSV");

    info!("Spawning classified texts writer");
    let (tx, rx) = mpsc::channel();
    let unclassified_texts_copy = unclassified_texts.clone();
    thread::spawn(|| {
        persistence::classified_writer(
            "classified.csv",
            headers,
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
