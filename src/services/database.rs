// TODO: add sqlite database connection. Add, delete and update functionalities.

use crate::models::{command, pipeline};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use std::env::current_dir;
use std::error::Error;

pub fn database_path() -> String {
    let mut path = current_dir().unwrap();
    path.push("sqlite.db");
    println!("this is the database {:?}", path);
    return path.to_str().unwrap().to_string();
}

pub async fn migrate_database(database_path: String) -> Result<SqlitePool, Box<dyn Error>> {
    let pool = SqlitePool::connect(database_path.as_str())
        .await
        .expect("could not connect to database.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("could not migrate database");
    Ok(pool)
}

pub async fn create_database(path_to_db: String) {
    if !Sqlite::database_exists(&path_to_db).await.unwrap() {
        std::fs::File::create(&path_to_db).expect("could not create database file");
        println!("the database does not exist {}", &path_to_db);
        Sqlite::create_database(&path_to_db)
            .await
            .expect("could not create database");
    }
}
pub async fn connect_database(path_to_db: String) -> Result<SqlitePool, Box<dyn Error>> {
    Ok(SqlitePool::connect(&path_to_db).await.unwrap())
}

pub async fn add_pipeline(
    pool: &SqlitePool,
    pipeline: pipeline::Pipeline,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO pipelines (id, description, class) VALUES ($1,$2,$3)")
        .bind(&pipeline.id)
        .bind(&pipeline.description)
        .bind(&pipeline.class)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn add_command(
    pool: &SqlitePool,
    pipeline_id: String,
    command: command::Command,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "INSERT INTO commands (id, command, sorting_order, pipeline_id) VALUES ($1,$2,$3,$4)",
    )
    .bind(&command.id)
    .bind(&command.command)
    .bind(&command.sorting_order)
    .bind(&pipeline_id)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn delete_all_commands(
    pool: &SqlitePool,
    pipeline_id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("DELETE FROM commands WHERE pipeline_id = $1")
        .bind(&pipeline_id)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn delete_specific_command(
    pool: &SqlitePool,
    pipeline_id: &String,
    sorting_index: &u32,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("DELETE FROM commands WHERE pipeline_id = $1 AND sorting_order = $2")
        .bind(pipeline_id)
        .bind(sorting_index)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_command(pool: &SqlitePool, id: &String) -> Result<(), Box<dyn Error>> {
    sqlx::query("DELETE FROM pipelines WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_pipeline(
    pool: &SqlitePool,
    id: &String,
) -> Result<pipeline::Pipeline, Box<dyn Error>> {
    // Query to get the command by id, deserialized into pipeline::Pipeline
    let command = sqlx::query_as::<_, pipeline::Pipeline>("SELECT * FROM pipelines WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(command)
}

pub async fn find_all_commands(
    pool: &SqlitePool,
    id: &String,
) -> Result<Vec<command::Command>, Box<dyn Error>> {
    let commands: Vec<command::Command> =
        sqlx::query_as("SELECT * FROM commands WHERE pipeline_id = $1")
            .bind(id)
            .fetch_all(pool)
            .await?;
    Ok(commands)
}
#[allow(dead_code)]
pub async fn modify_command(
    pool: &SqlitePool,
    new_command: &String,
    id: &String,
    index: u32,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE commands SET command = $1 WHERE sorting_order = $2 AND pipeline_id = $3")
        .bind(new_command)
        .bind(index)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}
//TODO: Replace this with switch of Sorting Orders
#[allow(dead_code)]
pub async fn change_command_index(
    pool: &SqlitePool,
    id: &String,
    index: u32,
    new_index: u32,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "UPDATE commands SET sorting_order = $1 WHERE sorting_order = $2 AND pipeline_id = $3",
    )
    .bind(new_index)
    .bind(index)
    .bind(id)
    .execute(pool)
    .await
    .unwrap();
    Ok(())
}
