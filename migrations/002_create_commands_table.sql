CREATE TABLE commands(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    command VARCHAR,
    sorting_order INT,
    pipeline_id VARCHAR,
    FOREIGN KEY(pipeline_id) REFERENCES pipelines(id)
);
