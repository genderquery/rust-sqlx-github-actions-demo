use sqlx::PgPool;

#[sqlx::test(fixtures("messages"))]
async fn test_message(pool: PgPool) -> sqlx::Result<()> {
    let row = sqlx::query!("select * from messages")
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.message, "Hello, world!");

    Ok(())
}
