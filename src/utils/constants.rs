use dotenvy::dotenv;
use std::env as std_env;
use std::sync::LazyLock;

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:8080";
    pub const DB_ADDRESS: &str = "127.0.0.1:5432";
    pub const TABLE_NAME: &str = "todos";
    pub const DB_MAX_CONNECTIONS: u32 = 10;
    pub const MAX_ROWS: u32 = 1000;
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub const DB_ADDRESS: &str = "127.0.0.1:5432";
    pub const TABLE_NAME: &str = "todos";
    pub const DB_MAX_CONNECTIONS: u32 = 10;
    pub const MAX_ROWS: u32 = 1000;
}

pub mod env {
    pub const DB_NAME_ENV_VAR: &str = "DB_NAME";
    pub const DB_USER_ENV_VAR: &str = "DB_USER";
    pub const PASSWORD_ENV_VAR: &str = "PASSWORD";
}

pub static DB_NAME_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::DB_NAME_ENV_VAR)
        .expect("DB_NAME_SECRET must be set.");
    if secret.is_empty() {
        panic!("DB_NAME_SECRET must not be empty.");
    }
    secret
});

pub static DB_USER_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::DB_USER_ENV_VAR)
        .expect("DB_USER_NAME_SECRET must be set.");
    if secret.is_empty() {
        panic!("DB_USER_NAME_SECRET must not be empty.");
    }
    secret
});

pub static PASSWORD_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::PASSWORD_ENV_VAR)
        .expect("PASSWORD_SECRET must be set.");
    if secret.is_empty() {
        panic!("PASSWORD_SECRET must not be empty.");
    }
    secret
});