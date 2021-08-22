//! CSV reading and writing.

use csv::Writer;
use eyre::Result;
use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    path::Path,
    sync::mpsc::Receiver,
};

use crate::{ClassifiedTextRequest, UnclassifiedText};

/// Returns all texts from `unclassified_path` that are not in
/// `classified_path` yet.
pub fn load_unclassified(
    unclassified_path: impl AsRef<Path>,
    classified_path: impl AsRef<Path>,
) -> Result<HashMap<String, UnclassifiedText>> {
    let classified_ids = get_classified_ids(classified_path);

    let mut unclassified_texts = HashMap::new();
    let mut reader = csv::Reader::from_path(unclassified_path)?;

    for record in reader.records().map(Result::unwrap).filter(|record| {
        classified_ids.get(&record[record.len() - 2]).is_none()
    }) {
        let record: Vec<_> = record.into_iter().collect();

        let detected_languages: Vec<String> = record[0..record.len() - 2]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let id = record[record.len() - 2].to_string();
        let text = record[record.len() - 1].to_string();

        let unclassified_text = UnclassifiedText {
            detected_languages,
            id,
            text,
        };

        unclassified_texts
            .insert(unclassified_text.id.clone(), unclassified_text);
    }

    Ok(unclassified_texts)
}

/// Returns the headers from the given CSV file.
pub fn get_headers(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let mut reader = csv::Reader::from_path(path)?;

    Ok(reader.headers()?.iter().map(str::to_owned).collect())
}

/// Worker that receives classified texts and writes them to CSV.
pub fn classified_writer(
    classified_path: impl AsRef<Path>,
    mut headers: Vec<String>,
    mut unclassified_texts: HashMap<String, UnclassifiedText>,
    rx: Receiver<ClassifiedTextRequest>,
) -> Result<()> {
    let file_existed = classified_path.as_ref().exists();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(classified_path)?;
    let mut writer = Writer::from_writer(file);

    if !file_existed {
        headers.push("human".into());
        writer.write_record(headers)?;
        writer.flush()?;
    }

    for classified_text in rx {
        if let Some(unclassified_text) =
            unclassified_texts.remove(&classified_text.id)
        {
            let mut record = unclassified_text.detected_languages;
            record.push(unclassified_text.id);
            record.push(unclassified_text.text);
            record.push(classified_text.language);

            writer.write_record(record)?;
            writer.flush()?;
        }
    }

    Ok(())
}

/// Returns the IDs of all texts from the given CSV file.
fn get_classified_ids(classified_path: impl AsRef<Path>) -> HashSet<String> {
    csv::Reader::from_path(classified_path)
        .map(|mut reader| {
            reader
                .records()
                .map(Result::unwrap)
                .map(|record| record[record.len() - 3].to_owned())
                .collect()
        })
        .unwrap_or_default()
}
