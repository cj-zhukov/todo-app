use std::error::Error;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod error;
pub mod postgres_repo;

pub struct DB {
    server: Pool<Postgres>,
    address: String
}

impl AsRef<Pool<Postgres>> for DB {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.server
    }
}

impl DB {
    fn new(server: Pool<Postgres>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn run_migrations(&self) -> Result<(), Box<dyn Error>> {
        sqlx::migrate!().run(self.as_ref()).await?;
        tracing::info!("run migrations for server {}", &self.address);
        Ok(())
    }

    pub async fn build(address: &str, user: &str, pwd: &str, db: &str, max_connections: u32) -> Result<Self, Box<dyn Error>> {
        let url = format!("postgres://{}:{}@{}/{}", user, pwd, address, db);
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;
        tracing::info!("established connection to server: {} db: {}", address, db);
        Ok(DB::new(pool, address.to_string()))
    }
}
