use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use sqlx::{migrate::Migrator, Error as SqlxError};
use crate::store::Store;

pub async fn migrate(pool: &Store) -> Result<(), SqlxError> {
    let migrations_dir = Path::new("./migrations");
    if !migrations_dir.exists() {
        fs::create_dir_all(migrations_dir)
            .map_err(|e| SqlxError::Configuration(e.into()))?;
    }
    let initial_migration = migrations_dir.join("01__initial.sql");
    if !initial_migration.exists() {
        let mut file = File::create(initial_migration)
            .map_err(|e| SqlxError::Configuration(e.into()))?;

        let default_sql = r#"
            CREATE TABLE IF NOT EXISTS blogs (
                id BIGSERIAL PRIMARY KEY,
                image TEXT,
                author TEXT NOT NULL,
                date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                likes BIGINT NOT NULL DEFAULT 0,
                bookmarks INT NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS texts (
                blog_id BIGINT PRIMARY KEY REFERENCES blogs(id) ON DELETE CASCADE,
                text TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS comments (
                id SERIAL PRIMARY KEY,
                blog_id BIGINT NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
                author TEXT NOT NULL,
                text TEXT NOT NULL,
                likes INT NOT NULL DEFAULT 0,
                date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
        "#;

        file.write_all(default_sql.trim().as_bytes())
            .map_err(|e| SqlxError::Configuration(e.into()))?;
    }

    let migrator = Migrator::new(migrations_dir).await?;
    migrator.run(&pool.connection).await?;

    Ok(())
}