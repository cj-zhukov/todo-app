use std::error::Error;

use todo_app::{
    Application, db::DB, utils::{constants::{DB_NAME_SECRET, DB_USER_SECRET, PASSWORD_SECRET, prod}, tracing::init_tracing} 
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing("todo.log").expect("Failed to initialize tracing");
    let db = DB::build(prod::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, &DB_NAME_SECRET, 10).await?;
    db.run_migrations().await?;
    let app = Application::build(prod::APP_ADDRESS, db).await?;
    app.run().await?;   
    Ok(())
}
