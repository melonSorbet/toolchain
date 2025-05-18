use sqlx::FromRow;

#[derive(Debug, FromRow,Clone)]
pub struct Pipeline {
    pub id: String,
    pub description: String,
    pub class:  String,
}
