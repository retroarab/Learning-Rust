use sqlx::{self, Error, SqlitePool};
pub async fn init_db() -> Result<SqlitePool, Error> {
    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&connection_pool).await?;
    Ok(connection_pool)
}
