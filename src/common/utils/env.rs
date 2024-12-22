use dotenv::dotenv;
use std::env;

pub fn get_env(key: &str, default: &str) -> String {
    dotenv().ok();
    env::var(key).unwrap_or_else(|_| default.to_string())
}
