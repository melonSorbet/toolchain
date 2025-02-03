use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Command {
    pub id: String,
    pub description: String,
    pub class:  String,
}
