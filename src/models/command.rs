use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Command {
    command_id: i32,
    description: String,
    subcommand_id: [String; 5],
    command_name: String,
}
