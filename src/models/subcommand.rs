use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Subcommand {
    subcommand_id: i32,
    command: String,
    sorting_order: u32,
}
