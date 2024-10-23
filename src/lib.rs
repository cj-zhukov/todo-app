pub mod config;
pub mod db;
pub mod routes;
pub mod utils;

use config::Config;
use routes::{alive::ping, todos::*};

use anyhow::Result;
use axum::{routing::get, serve::Serve, Router};
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(config: Config, address: &str) -> Result<Self> {
        // let address = format!("{}:{}", config.app_server, config.app_port);
        
        let url = format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_password, config.db_server, config.db_port, config.db_name);
        let pool = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .connect(&url)
            .await?;
        println!("established connection to db: {} port: {}", config.db_name, config.app_port);

        sqlx::migrate!().run(&pool).await?;

        let router = Router::new()
            .route("/foo", get(|| async { "foo" }))
            .route("/alive", get(ping))
            .route("/todos", get(todo_list).post(todo_create))
            .route("/todos/:id", get(todo_read).put(todo_update).delete(todo_delete))
            .with_state(pool);

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