CREATE TABLE subcommands(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    command VARCHAR,
    sorting_order INT,
    command_id VARCHAR,
    FOREIGN KEY(command_id) REFERENCES commands(id)
);