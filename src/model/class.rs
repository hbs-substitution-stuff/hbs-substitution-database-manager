use std::error::Error;
use sqlx::{query, SqlitePool};

pub struct Class<'a> {
	pub(crate) name: &'a str,
}

impl Class<'_> {
	pub(crate) async fn insert(&self, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
		let mut conn = pool.acquire().await?;

		query!(
			r#"
INSERT OR IGNORE INTO class (name) VALUES (?1)
			"#,
			self.name,
		).execute(&mut conn).await?;

		Ok(())
	}
}