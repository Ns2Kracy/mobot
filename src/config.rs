use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surrealdb {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redis {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    surrealdb: Surrealdb,
    redis: Redis,
}

impl Config {
    pub fn load_config() {}

    pub fn from_config() {}
}
