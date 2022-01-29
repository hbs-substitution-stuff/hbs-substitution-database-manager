use std::error::Error;
use sqlx::{query, SqlitePool};
use crate::model::block::Block;
use crate::model::text::Text;

pub struct Substitution<'a> {
	block: Block,
	substitution: Vec<Text<'a>>,
}

impl<'a> Substitution<'a> {
	pub(crate) fn new(block: Block, sub: &'a Vec<String>) -> Self {
		Substitution {
			block,
			substitution: sub.iter()
				.map(|content| Text {
					content
				}).collect()
		}
	}

	pub(crate) async fn insert(&self, pool: &SqlitePool, pdf_pkey: &str, class_pkey: &str) -> Result<(), Box<dyn Error>> {
		let mut conn = pool.acquire().await?;
		let block = self.block as i64;

		let sub_pkey = query!(
			r#"
INSERT INTO substitution (block, class_name, pdf_hash) VALUES (?1, ?2, ?3)
			"#,
			block,
			class_pkey,
			pdf_pkey,
		).execute(&mut conn)
		.await?
		.last_insert_rowid();

		for text in &self.substitution {
			text.insert(pool, sub_pkey).await?;
		}

		Ok(())
	}
}