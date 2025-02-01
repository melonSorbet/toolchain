// TODO: add sqlite database connection. Add, delete and update functionalities.

use sqlx::{Error, SqlitePool};


pub async fn migrate_database() -> Result<(SqlitePool), Error> {
    let pool = SqlitePool::connect("sqlite:/mnt/c/Users/User/RustroverProjects/toolchain/mydb.db").await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok((pool))
}
