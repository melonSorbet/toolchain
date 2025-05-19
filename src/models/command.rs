use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct Command {
    pub id: Option<i32>,
    pub command: String,
    pub sorting_order: u32,
    pub pipeline_id: String,
}
