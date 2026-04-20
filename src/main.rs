use std::{error::Error, sync::Arc};

use todo_app::{
    Application,
    db::{DB, postgres_repo::PgTodoRepository},
    hm::hashmap_repo::HmTodoRepository,
    utils::{constants::{DB_NAME_SECRET, DB_USER_SECRET, PASSWORD_SECRET, prod},
    tracing::init_tracing} 
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing("todo.log").expect("Failed to initialize tracing");

    // run with postgres
    let db = DB::build(prod::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, &DB_NAME_SECRET, 10).await?;
    db.run_migrations().await?;
    let repo = Arc::new(PgTodoRepository::new(db.as_ref().clone()));
    let app = Application::build(prod::APP_ADDRESS, repo).await?;

    // run with hashmap
    // let repo = Arc::new(HmTodoRepository::new());
    // let app = Application::build(prod::APP_ADDRESS, repo).await?;
    
    app.run().await?;   
    Ok(())
}
