mod model;

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use hbs_table_extractor::HbsTableExtractor;
use reqwest::blocking::Request;
use sha2::{Sha256, Sha512};
use sha2::Digest;
use sqlx::SqlitePool;
use model::pdf::Pdf;
use crate::model::block::Block;
use crate::model::class::Class;
use crate::model::entry::Entry;
use crate::model::substitution::Substitution;


#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    achieve_pdf("./mos.pdf", 0, &pool).await.unwrap();
}

async fn achieve_pdf<'a, T: AsRef<Path> + AsRef<OsStr>>(path: T, fetched_at: i64, pool: &SqlitePool) -> Result<(), Box<dyn Error>>
    where String: From<T> {
    let (pdf_entry, data) = {
        let pdf_file = OpenOptions::new().read(true).open(&path)?;
        let mut pdf_extractor = HbsTableExtractor::load_from(&pdf_file)?;
        let hasher = Sha512::new_with_prefix(
            &pdf_file.bytes().collect::<Result<Vec<u8>, std::io::Error>>()?
        );

        (
            Pdf {
                date: pdf_extractor.extract_date()?,
                fetched_at,
                hash: format!("{:x}", hasher.finalize()),
                source: Some(path.into()),
            },
            pdf_extractor.extract_tables()?
                .into_iter().flatten().flatten()
                .collect::<Vec<Vec<Vec<String>>>>()
        )
    };

    let mut entries = Vec::new();
    let mut classes = data.iter()
        .map(|c| Class {
            name: c[0][0].as_str(),
        }).collect::<Vec<Class>>();

    for column in 0..data.len() {
        for cell in 1..data[column].len() {
            entries.push(
                Entry {
                    pdf: &pdf_entry,
                    class: &classes[column],
                    substitution: Substitution::new(
                        Block::try_from(cell as i64 - 1)?,
                        &data[column][cell]
                    )
                }
            );
        }
    }

    for entry in entries {
        entry.insert(pool).await?;
    }

    Ok(())
}

// Basic aGJzdXNlcjpoYnNwYXNz
fn download_pdf() -> Result<(), Box<dyn Error>> {
    todo!()
}