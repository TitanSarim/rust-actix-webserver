use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {

    MySqlPool::connect("mysql://root:12345@localhost:3306/activeweb").await
}