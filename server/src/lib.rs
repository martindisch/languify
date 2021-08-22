use serde::{Deserialize, Serialize};

pub mod handlers;
pub mod persistence;

// TODO: move more logic into impls on this type
#[derive(Debug, Clone)]
pub struct UnclassifiedText {
    pub detected_languages: Vec<String>,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct TextResponse<'a> {
    pub id: usize,
    pub text: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ClassificationRequest {
    pub id: usize,
    pub language: String,
}
