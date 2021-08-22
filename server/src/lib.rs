use serde::{Deserialize, Serialize};

pub mod handlers;
pub mod persistence;

/// An unclassified text.
#[derive(Debug, Clone)]
pub struct UnclassifiedText {
    pub detected_languages: Vec<String>,
    pub id: String,
    pub text: String,
}

/// An unclassified text for sending to the client.
#[derive(Debug, Serialize)]
pub struct UnclassifiedTextResponse<'a> {
    pub id: &'a str,
    pub text: &'a str,
}

/// A classified text from the client.
#[derive(Debug, Deserialize)]
pub struct ClassifiedTextRequest {
    pub id: String,
    pub language: String,
}
