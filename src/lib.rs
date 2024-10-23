use anyhow::Result;
use axum::{routing::get, serve::Serve, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod db;
pub mod routes;
pub mod utils;

use routes::{alive::ping, todos::*};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(address: &str, db: DB) -> Result<Self> {        
        let router = Router::new()
            .route("/foo", get(|| async { "foo" }))
            .route("/alive", get(ping))
            .route("/todos", get(todo_list).post(todo_create))
            .route("/todos/:id", get(todo_read).put(todo_update).delete(todo_delete))
            .with_state(db.server);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<()> {
        println!("listening on {}", &self.address);
        self.server.await?;

        Ok(())
    }
}

pub struct DB {
    server: Pool<Postgres>,
    pub address: String
}

impl DB {
    fn new(server: Pool<Postgres>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!().run(&self.server).await?;
        println!("run migrations for server: {}", &self.address);

        Ok(())
    }

    pub async fn build(address: &str, user: &str, pwd: &str, db: &str, max_connections: u32) -> Result<Self> {
        let url = format!("postgres://{}:{}@{}/{}", user, pwd, address, db);
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        println!("established connection to server: {} db: {}", address, db);

        Ok(DB::new(pool, address.to_string()))
    }
}