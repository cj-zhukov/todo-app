use std::error::Error;

use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use uuid::Uuid;

use todo_app::{
    utils::constants::{test, DATABASE_URL, DB_USER_SECRET, PASSWORD_SECRET}, 
    Application, 
    DB
};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub db_name: String,
}

impl TestApp {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let db_name = Uuid::new_v4().to_string();
        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, "postgres", 10).await?;
        db
            .server
            .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
            .await
            .expect("Failed to create database.");
        db.run_migrations().await?;
    
        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, &db_name, 10).await?;
        let app = Application::build(test::APP_ADDRESS, db).await?;
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = Client::new();

        Ok(Self { address, http_client, db_name })
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

    // pub async fn post_create_todo<Body>(&self, body: &Body) -> reqwest::Response
    // where Body: serde::Serialize,
    // {
    //     self.http_client
    //         .post(&format!("{}/signup", &self.address))
    //         .json(body)
    //         .send()
    //         .await
    //         .expect("Failed to execute request.")
    // }

    pub async fn cleanup(&self) {
        let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, "postgres", 10)
            .await
            .expect("failed creating pool");
        
        db.server
            .execute(format!(r#"drop database "{}" with (force);"#, self.db_name).as_str())
            .await
            .expect("failed to drop the database");
    }
}