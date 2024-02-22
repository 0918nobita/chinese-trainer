use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
struct User {
    id: i64,
    name: String,
    email: String,
    address: Option<String>,
    created_at: chrono::NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().expect("Failex to read .env file");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&database_url).await?;

    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
