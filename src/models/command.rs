use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Command {
    command_id: i32,
    description: String,
    class:  String,
}
