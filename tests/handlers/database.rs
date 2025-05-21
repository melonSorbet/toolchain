#[cfg(test)]
mod tests {

    use sqlx::Row;
    use toolchain::services::database;
    use toolchain::{models, services};

    #[tokio::test]
    async fn test_database_tables() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // For in-memory database, we don't need create_database
        let pool = database::migrate_database(test_db_url.to_string()).await?;

        // Verify migrations
        let tables = sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )
        .fetch_all(&pool)
        .await?;

        // Should at least have the _sqlx_migrations table
        assert!(tables.contains(&"pipelines".to_string()));
        assert!(tables.contains(&"commands".to_string()));
        assert!(!tables.contains(&"random table name ".to_string()));
        Ok(())
    }
    #[tokio::test]
    async fn test_commands_rows() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // For in-memory database, we don't need create_database
        let pool = database::migrate_database(test_db_url.to_string()).await?;

        let columns = sqlx::query("PRAGMA table_info(commands)")
            .fetch_all(&pool)
            .await?;

        println!("Commands table columns:");
        for column in &columns {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            println!("- {} ({})", name, type_);
        }
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "id" && type_ == "INTEGER"
        }));
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "command" && type_ == "VARCHAR"
        }));
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "sorting_order" && type_ == "INT"
        }));

        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "pipeline_id" && type_ == "VARCHAR"
        }));
        Ok(())
    }
    #[tokio::test]
    async fn test_pipelines_rows() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // For in-memory database, we don't need create_database
        let pool = database::migrate_database(test_db_url.to_string()).await?;

        let columns = sqlx::query("PRAGMA table_info(pipelines)")
            .fetch_all(&pool)
            .await?;

        println!("Pipelines table columns:");
        for column in &columns {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            println!("- {} ({})", name, type_);
        }
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "description" && type_ == "VARCHAR"
        }));
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "id" && type_ == "VARCHAR"
        }));
        assert!(columns.iter().any(|column| {
            let name: String = column.get("name");
            let type_: String = column.get("type");
            name == "class" && type_ == "VARCHAR"
        }));
        Ok(())
    }
    #[tokio::test]
    async fn add_pipelines_into_database() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };

        database::add_pipeline(&pool, pipeline.clone())
            .await
            .unwrap();
        // Verify with raw query
        let (id, description, class): (String, String, String) =
            sqlx::query_as("SELECT id, description, class FROM pipelines WHERE id = ?")
                .bind(&pipeline.id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(id, pipeline.id);
        assert_eq!(description, pipeline.description);
        assert_eq!(class, pipeline.class);
        Ok(())
    }
    #[tokio::test]
    async fn find_pipeline_from_database() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };

        database::add_pipeline(&pool, pipeline.clone())
            .await
            .unwrap();
        // Verify with raw query
        let result = database::find_pipeline(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(result.id, pipeline.id);
        assert_eq!(result.description, pipeline.description);
        assert_eq!(result.class, pipeline.class);
        Ok(())
    }
    #[tokio::test]
    async fn delete_pipeline_from_database() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };

        database::add_pipeline(&pool, pipeline.clone())
            .await
            .unwrap();
        // Verify with raw query
        database::delete_command(&pool, &"test".to_string())
            .await
            .unwrap();
        let result: Result<models::pipeline::Pipeline, _> =
            sqlx::query_as::<_, models::pipeline::Pipeline>(
                "SELECT id, description, class FROM pipelines WHERE id = ?",
            )
            .bind(&pipeline.id)
            .fetch_one(&pool)
            .await;
        assert!(result.is_err());
        Ok(())
    }
    #[tokio::test]
    async fn add_command_to_database() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };

        database::add_pipeline(&pool, pipeline.clone())
            .await
            .unwrap();
        let command = models::command::Command {
            id: None,
            command: "ls".to_string(),
            sorting_order: 1,
            pipeline_id: "test".to_string(),
        };

        database::add_command(&pool, "test".to_string(), command.clone())
            .await
            .unwrap();

        let result: models::command::Command = sqlx::query_as::<_, models::command::Command>(
            "SELECT id, command, sorting_order, pipeline_id FROM commands WHERE pipeline_id = ?",
        )
        .bind("test")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(result.command, "ls");
        assert_eq!(result.sorting_order, 1);
        Ok(())
    }
    #[tokio::test]
    async fn find_all_commands() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };

        services::database::add_pipeline(&pool, pipeline)
            .await
            .unwrap();
        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "ls".to_string(),
                sorting_order: 1,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "mvn".to_string(),
                sorting_order: 2,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(commands[0].command, "ls");
        assert_eq!(commands[1].command, "mvn");
        Ok(())
    }

    #[tokio::test]
    async fn delete_all_commands() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };
        services::database::add_pipeline(&pool, pipeline)
            .await
            .unwrap();

        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "ls".to_string(),
                sorting_order: 1,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "mvn".to_string(),
                sorting_order: 2,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(commands.len(), 2);
        database::delete_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(commands.len(), 0);
        Ok(())
    }
    #[tokio::test]
    async fn modify_command() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };
        services::database::add_pipeline(&pool, pipeline)
            .await
            .unwrap();

        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "ls".to_string(),
                sorting_order: 1,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "mvn".to_string(),
                sorting_order: 2,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();
        assert_eq!(commands[1].command, "mvn");

        database::modify_command(&pool, &"ripgrep".to_string(), &"test".to_string(), 2)
            .await
            .unwrap();

        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();
        assert_eq!(commands[1].command, "ripgrep");
        Ok(())
    }
    #[tokio::test]
    async fn modify_command_sorting_index() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string())
            .await
            .unwrap();
        let pipeline: models::pipeline::Pipeline = models::pipeline::Pipeline {
            id: "test".to_string(),
            description: "test_description".to_string(),
            class: "test_class".to_string(),
        };
        services::database::add_pipeline(&pool, pipeline)
            .await
            .unwrap();

        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "ls".to_string(),
                sorting_order: 1,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        database::add_command(
            &pool,
            "test".to_string(),
            models::command::Command {
                id: None,
                command: "mvn".to_string(),
                sorting_order: 2,
                pipeline_id: "test".to_string(),
            },
        )
        .await
        .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(commands[1].sorting_order, 2);

        database::change_command_index(&pool, &"test".to_string(), 2, 3)
            .await
            .unwrap();
        let commands = database::find_all_commands(&pool, &"test".to_string())
            .await
            .unwrap();

        assert_eq!(commands[1].sorting_order, 3);
        Ok(())
    }
}
