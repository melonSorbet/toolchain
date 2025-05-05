// TODO: add sqlite database connection. Add, delete and update functionalities.

use crate::models::{command, pipeline};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use std::env::current_dir;
use std::error::Error;

pub fn database_path() -> String {
    let mut path = current_dir().unwrap();
    path.push("sqlite.db");
    return path.to_str().unwrap().to_string();
}

pub async fn migrate_database() -> Result<(SqlitePool), Box<dyn Error>> {
    let pool = SqlitePool::connect(database_path().as_str())
        .await
        .expect("could not connect to database.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("could not migrate database");
    Ok(pool)
}

pub async fn create_database() {
    let path_to_db = database_path();
    std::fs::File::create(&path_to_db).expect("could not create database file");
    if !Sqlite::database_exists(&path_to_db).await.unwrap() {
        println!("the database does not exist {}", &path_to_db);
        Sqlite::create_database(&path_to_db)
            .await
            .expect("could not create database");
    }
}
pub async fn connect_database() -> Result<SqlitePool, Box<dyn Error>> {
    let path_to_db = database_path();
    let pool = SqlitePool::connect(&path_to_db).await.unwrap();
    Ok(pool)
}

pub async fn add_command(
    pool: &SqlitePool,
    command: pipeline::Pipeline,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO pipelines (id, description, class) VALUES ($1,$2,$3)")
        .bind(&command.id)
        .bind(&command.description)
        .bind(&command.class)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn add_subcommand(
    pool: &SqlitePool,
    pipeline_id: String,
    subcommand: command::Command,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "INSERT INTO commands (id, command, sorting_order, pipeline_id) VALUES ($1,$2,$3,$4)",
    )
    .bind(&subcommand.id)
    .bind(&subcommand.command)
    .bind(&subcommand.sorting_order)
    .bind(&pipeline_id)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn delete_all_subcommands(
    pool: &SqlitePool,
    pipeline_id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("DELETE FROM commands WHERE pipeline_id = $1")
        .bind(&pipeline_id)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn delete_specific_subcommand(
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

pub async fn find_command(
    pool: &SqlitePool,
    id: &String,
) -> Result<pipeline::Pipeline, Box<dyn Error>> {
    // Query to get the command by id, deserialized into pipeline::Pipeline
    let command = sqlx::query_as::<_, pipeline::Pipeline>("SELECT * FROM pipelines WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    // Return the found command
    Ok(command)
}

pub async fn find_all_subcommands(
    pool: &SqlitePool,
    id: &String,
) -> Result<Vec<command::Command>, Box<dyn Error>> {
    let subcommands: Vec<command::Command> =
        sqlx::query_as("SELECT * FROM Commands WHERE pipeline_id = $1")
            .bind(id)
            .fetch_all(pool)
            .await
            .unwrap(); // Or handle error as needed
    Ok(subcommands)
}

pub async fn modify_subcommand(
    pool: &SqlitePool,
    new_command: &String,
    id: &String,
    index: u32,
) -> Result<Vec<command::Command>, Box<dyn Error>> {
    let subcommands: Vec<command::Command> =
        sqlx::query_as("UPDATE commands SET command = $1 WHERE sorting_order = $2 AND id = $3")
            .bind(new_command)
            .bind(index)
            .bind(id)
            .fetch_all(pool)
            .await
            .unwrap();
    Ok(subcommands)
}

pub async fn change_subcommand_index(
    pool: &SqlitePool,
    id: &String,
    index: u32,
    new_index: u32,
) -> Result<Vec<command::Command>, Box<dyn Error>> {
    let subcommands: Vec<command::Command> = sqlx::query_as(
        "UPDATE commands SET sorting_order = $1 WHERE sorting_order = $2 AND id = $3",
    )
    .bind(new_index)
    .bind(index)
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    Ok(subcommands)
}
