use std::{error::Error, sync::Arc};

use reqwest::Client;
use sqlx::Executor;
use uuid::Uuid;

use todo_app::{
    Application, domain::todo_repository::TodoRepository, infrastructure::{hashmap::hashmap_repo::HmTodoRepository, postgres::{DB, postgres_repo::PgTodoRepository}}, utils::constants::{DB_USER_SECRET, PASSWORD_SECRET, test} 
};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub db_name: String,
}

impl TestApp {
    pub async fn new_postgres() -> Result<Self, Box<dyn Error>> {
        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, "postgres", 10).await?;
        let db_name = Uuid::new_v4().to_string();
        db
            .as_ref()
            .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
            .await
            .expect("Failed to create database.");

        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, &db_name, 10).await?;
        db.run_migrations().await?;
        let repo = Arc::new(PgTodoRepository::new(db.as_ref().clone()));
        Self::spawn_app(repo, Some(db_name)).await
    }

    pub async fn new_hashmap() -> Result<Self, Box<dyn Error>> {
        let repo = Arc::new(HmTodoRepository::new());
        Self::spawn_app(repo, None).await
    }

    async fn spawn_app(
        repo: Arc<dyn TodoRepository>,
        db_name: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let app = Application::build(test::APP_ADDRESS, repo).await?;
        let address = format!("http://{}", app.address.clone());

        let _ = tokio::spawn(app.run());

        Ok(Self {
            address,
            http_client: Client::new(),
            db_name: db_name.unwrap_or_default(),
        })
    }

    pub async fn get_alive(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/alive", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_todos(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/todos", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_create_todo<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/todos", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_read_id(&self, id: i64) -> reqwest::Response
    {
        self.http_client
            .post(&format!("{}/todos:id", &self.address))
            .json(&id)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn cleanup(&self) {
        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, "postgres", 10)
            .await
            .expect("failed creating pool");
        
        db
            .as_ref()
            .execute(format!(r#"drop database "{}" with (force);"#, self.db_name).as_str())
            .await
            .expect("failed to drop the database");
    }
}
