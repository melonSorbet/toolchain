CREATE TABLE subcommands(
    id VARCHAR not null PRIMARY KEY,
    command VARCHAR,
    sorting_order INT,
    command_id VARCHAR,
    FOREIGN KEY(command_id) REFERENCES commands(id)
);