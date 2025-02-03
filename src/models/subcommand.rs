use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Subcommand {
    pub subcommand_id: String,
    pub command: String,
    pub sorting_order: u32,
}
