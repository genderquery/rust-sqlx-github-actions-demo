use sqlx::{Pool, Postgres};

pub fn message() -> &'static str {
    "Hello, world!"
}

pub async fn get_message_from_db(pool: Pool<Postgres>) -> Result<String, sqlx::Error> {
    let row = sqlx::query!("select * from messages")
        .fetch_one(&pool)
        .await?;
    Ok(row.message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        assert_eq!(message(), "Hello, world!")
    }
}
