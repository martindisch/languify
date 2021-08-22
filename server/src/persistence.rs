use eyre::Result;
use std::{collections::HashMap, path::Path};

pub fn load_unclassified(
    path: impl AsRef<Path>,
) -> Result<HashMap<usize, UnclassifiedText>> {
    let mut unclassified_texts = HashMap::new();
    let mut reader = csv::Reader::from_path(path)?;

    for (i, result) in reader.records().enumerate() {
        let record = result?;
        let detected_languages = record
            .iter()
            .take(record.len() - 2)
            .map(str::to_owned)
            .collect();
        let text = record.iter().last().unwrap().to_owned();

        let unclassified_text = UnclassifiedText {
            detected_languages,
            text,
        };

        unclassified_texts.insert(i, unclassified_text);
    }

    Ok(unclassified_texts)
}

#[derive(Debug)]
pub struct UnclassifiedText {
    pub detected_languages: Vec<String>,
    pub text: String,
}
