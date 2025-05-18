
#[cfg(test)]
mod tests {

    use toolchain::services::database;
    use toolchain::models;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use std::str::FromStr;
    use sqlx::Row;
    
    #[tokio::test]
    async fn test_database_tables() -> Result<(), Box<dyn std::error::Error>> {
        let test_db_url = "sqlite::memory:?cache=shared";

        // For in-memory database, we don't need create_database
        let pool = database::migrate_database(test_db_url.to_string()).await?;

        // Verify migrations
        let tables = sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
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
    async fn add_into_database() -> Result<(), Box<dyn std::error::Error>>{
        let test_db_url = "sqlite::memory:?cache=shared";
        
        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string()).await.unwrap();
        let pipeline: models::pipeline::Pipeline =
            models::pipeline::Pipeline{id:"test".to_string(),description:"test_description".to_string() ,class:"test_class".to_string()};

        database::add_command(&pool,pipeline.clone()).await.unwrap();
        // Verify with raw query
        let (id, description, class): (String, String, String) = sqlx::query_as(
            "SELECT id, description, class FROM pipelines WHERE id = ?"
        )
        .bind(&pipeline.id)
        .fetch_one(&pool)
        .await.unwrap();

        assert_eq!(id, pipeline.id);
        assert_eq!(description, pipeline.description);
        assert_eq!(class, pipeline.class);
        Ok(())
    }
    #[tokio::test]
    async fn find_from_database() -> Result<(), Box<dyn std::error::Error>>{
        let test_db_url = "sqlite::memory:?cache=shared";
        
        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string()).await.unwrap();
        let pipeline: models::pipeline::Pipeline =
            models::pipeline::Pipeline{id:"test".to_string(),description:"test_description".to_string() ,class:"test_class".to_string()};

        database::add_command(&pool,pipeline.clone()).await.unwrap();
        // Verify with raw query
        let result = database::find_command(&pool,&"test".to_string()).await.unwrap();

        assert_eq!(result.id, pipeline.id);
        assert_eq!(result.description, pipeline.description);
        assert_eq!(result.class, pipeline.class);
        Ok(())
    }
    #[tokio::test]
    async fn delete_from_database() -> Result<(), Box<dyn std::error::Error>>{
        let test_db_url = "sqlite::memory:?cache=shared";
        
        // Test database creation
        let pool = database::migrate_database(test_db_url.to_string()).await.unwrap();
        let pipeline: models::pipeline::Pipeline =
            models::pipeline::Pipeline{id:"test".to_string(),description:"test_description".to_string() ,class:"test_class".to_string()};

        database::add_command(&pool,pipeline.clone()).await.unwrap();
        // Verify with raw query
        database::delete_command(&pool,&"test".to_string()).await.unwrap();
        let result: Result<models::pipeline::Pipeline,_>= sqlx::query_as::<_, models::pipeline::Pipeline>(
            "SELECT id, description, class FROM pipelines WHERE id = ?"
        )
        .bind(&pipeline.id)
        .fetch_one(&pool)
        .await;
        assert!(result.is_err());
        Ok(())
    }

}
