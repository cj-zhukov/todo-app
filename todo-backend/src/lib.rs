use axum::{Router, http::{HeaderValue, Method}, routing::get, serve::Serve};
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};

pub mod infrastructure;
pub mod error;
pub mod domain;
pub mod routes;
pub mod utils;

use std::{error::Error, sync::Arc};

use routes::{alive::ping, todos::*};
use utils::tracing::*;
use domain::todo_repository::TodoRepository;

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

    pub async fn build(address: &str, repo: Arc<dyn TodoRepository>) -> Result<Self, Box<dyn Error>> {        
        let state = AppState { repo };
        // let cors = CorsLayer::new()
        //     .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        //     .allow_headers(Any);
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        let router = Router::new()
            .route("/", get(|| async { "Todo App" }))
            .route("/alive", get(ping))
            .route("/todos", get(todo_list).post(todo_create))
            .route("/todos/:id", get(todo_read).put(todo_update).delete(todo_delete))
            .with_state(state)
            .layer(cors)
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

