use std::error::Error;
use sqlx::{query, SqlitePool};

pub struct Text<'a> {
	pub(crate) content: &'a String
}

impl Text<'_> {
	pub(crate) async fn insert(&self, pool: &SqlitePool, sub_pkey: i64) -> Result<(), Box<dyn Error>> {
		let mut conn = pool.acquire().await?;

		query!(
			r#"
INSERT INTO substitution_content (content, substitution_id) VALUES (?1, ?2)
			"#,
			self.content,
			sub_pkey,
		).execute(&mut conn).await?;

		Ok(())
	}
}