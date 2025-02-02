// TODO: add sqlite database connection. Add, delete and update functionalities.

use sqlx::SqlitePool;
use std::error::Error;
use crate::models::{command, subcommand};

pub async fn migrate_database() -> Result<(SqlitePool), Box<dyn Error>> {
    let pool = SqlitePool::connect("sqlite:/mnt/c/Users/User/RustroverProjects/toolchain/mydb.db")
        .await
        .expect("whaat");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("TODO: panic message");
    Ok((pool))
}

pub async fn add_command(
    pool: &SqlitePool,
    command: command::Command,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO commands (id, description, class) VALUES ($1,$2,$3)")
        .bind(&command.id)
        .bind(&command.description)
        .bind(&command.class)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn add_subcommand(
    pool: &SqlitePool,
    command_id: String,
    subcommand: subcommand::Subcommand,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "INSERT INTO subcommands (id, command, sorting_order, command_id) VALUES ($1,$2,$3,$4)",
    )
    .bind(&subcommand.subcommand_id)
    .bind(&subcommand.command)
    .bind(&subcommand.sorting_order)
    .bind(&command_id)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn delete_all_subcommands(pool: &SqlitePool, command_id: String) -> Result<(),Box<dyn Error>> {
    sqlx::query("DELETE FROM subcommands WHERE command_id = $1")
        .bind(&command_id)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn delete_specific_subcommand(pool: &SqlitePool, command_id: String, sorting_index: u32) -> Result<(),Box<dyn Error>> {
    sqlx::query("DELETE FROM subcommands WHERE command_id = $1 AND sorting_index = $2")
        .bind(command_id)
        .bind(sorting_index)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn delete_command(pool: &SqlitePool, id: String) -> Result<(),Box<dyn Error>> {
    sqlx::query("DELETE FROM commands WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_command(pool: &SqlitePool, id: String) -> Result<command::Command, Box<dyn Error>> {
    // Query to get the command by id, deserialized into command::Command
    let command = sqlx::query_as::<_, command::Command>("SELECT * FROM commands WHERE id = $1")
        .bind(&id)
        .fetch_one(pool)
        .await?;

    // Return the found command
    Ok(command)
}

/*pub async fn find_all_subcommands(pool: &SqlitePool, id: String) ->Result<(subcommand::Subcommand), Box<dyn Error>>{
    /*let subcommand: subcommand::Subcommand = sqlx::query("SELECT * FROM subcommands WHERE command_id = $1")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok((subcommand))*/
}*/
