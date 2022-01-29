use std::error::Error;
use sqlx::{query, SqlitePool};
use crate::model::class::Class;
use crate::model::pdf::Pdf;
use crate::model::substitution::Substitution;

pub struct Entry<'a> {
	pub(crate) pdf: &'a Pdf,
	pub(crate) class: &'a Class<'a>,
	pub(crate) substitution: Substitution<'a>,
}

impl<'a> Entry<'a> {
	pub(crate) async fn insert(&self, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
		self.pdf.insert(pool).await?;
		self.class.insert(pool).await?;

		let mut conn = pool.acquire().await?;

		query!(
			r#"
INSERT OR IGNORE INTO has (pdf_hash, class_name) VALUES (?1, ?2)
			"#,
			self.pdf.hash,
			self.class.name,
		).execute(&mut conn).await?;

		self.substitution.insert(pool, &self.pdf.hash, &self.class.name).await
	}
}
