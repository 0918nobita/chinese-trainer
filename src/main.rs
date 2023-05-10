use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    // データベースが存在するか確認し、存在しない場合は作成する
    if Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Database already exists")
    } else {
        println!("Creating database...");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created successfully"),
            Err(error) => panic!("error: {}", error),
        }
    }

    // データベースに接続し、テーブルを作成する
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query("CREATE TABLE IF NOT EXISTS words (id INTEGER PRIMARY KEY NOT NULL, hanzi TEXT NOT NULL, pinyin TEXT NOT NULL, meaning TEXT NOT NULL)").execute(&db).await.unwrap();
    println!("Result: {:?}", result)
}
