use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub const MAX_ROWS: u32 = 100;
const TABLE_NAME: &str = "todos";

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub fn table_name() -> String {
        TABLE_NAME.to_string()
    }
}