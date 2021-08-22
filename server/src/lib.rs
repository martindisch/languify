use serde::{Deserialize, Serialize};

pub mod handlers;
pub mod persistence;

#[derive(Debug, Clone)]
pub struct UnclassifiedText {
    pub detected_languages: Vec<String>,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct TextResponse<'a> {
    pub id: &'a str,
    pub text: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ClassificationRequest {
    pub id: String,
    pub language: String,
}
