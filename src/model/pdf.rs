use std::error::Error;
use sqlx::{query, SqlitePool};

pub struct Pdf {
	pub(crate) date: i64,
	pub(crate) fetched_at: i64,
	pub(crate) hash: String,
	pub(crate) source: Option<String>,
}

impl Pdf {
	pub(crate) async fn insert(&self, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
		let mut conn = pool.acquire().await?;

		query!(
			r#"
INSERT OR IGNORE INTO pdf (date, fetched_at, hash, source) VALUES (?1, ?2, ?3, ?4)
			"#,
			// convert date to string (yyyy-mm-dd hh:MM:ss.mm)
			self.date,
			self.fetched_at,
			self.hash,
			self.source,
		).execute(&mut conn).await?;

		Ok(())
	}
}