use todo_app::{
    utils::constants::test, Application, DB
};

use anyhow::Result;
use reqwest::Client;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Result<Self> {
        let db = DB::build(test::DB_ADDRESS, "postgres", "postgres", "demo", 10).await?;
        db.run_migrations().await?;
        let app = Application::build(test::APP_ADDRESS, db).await?;

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = Client::new();

        Ok(Self { address, http_client })
    }

    pub async fn get_alive(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/alive", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_create_todo<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}