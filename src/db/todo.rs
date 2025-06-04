use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub body: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
