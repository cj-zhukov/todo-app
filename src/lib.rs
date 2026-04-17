use axum::{routing::get, serve::Serve, Router};
use tower_http::trace::TraceLayer;

pub mod db;
pub mod error;
pub mod domain;
pub mod routes;
pub mod utils;

use std::{error::Error, sync::Arc};
use routes::{alive::ping, todos::*};
use utils::tracing::*;

use crate::{db::{DB, postgres_repo::PgTodoRepository}, domain::todo_repository::TodoRepository};

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn TodoRepository>,
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(address: &str, db: DB) -> Result<Self, Box<dyn Error>> {        
        let repo = PgTodoRepository::new(db.as_ref().clone());
        let state = AppState {
            repo: Arc::new(repo),
        };
        let router = Router::new()
            .route("/", get(|| async { "Todo App" }))
            .route("/alive", get(ping))
            .route("/todos", get(todo_list).post(todo_create))
            .route("/todos/:id", get(todo_read).put(todo_update).delete(todo_delete))
            .with_state(state)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await?;
        Ok(())
    }
}

