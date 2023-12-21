use github_actions_demo::get_message_from_db;
use sqlx::PgPool;

#[sqlx::test(fixtures("messages"))]
async fn test_message(pool: PgPool) -> sqlx::Result<()> {
    let message = get_message_from_db(pool).await?;
    assert_eq!(message, "Hello, world!");
    Ok(())
}
