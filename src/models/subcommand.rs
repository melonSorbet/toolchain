use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Subcommand {
    pub id: Option<i32>,
    pub command: String,
    pub sorting_order: u32,
    pub command_id: String
}
