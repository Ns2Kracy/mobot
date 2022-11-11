use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::path::Path;
use std::process::exit;

const CONFIG_FILE_PATH: &'static str = "bot.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub uin: i64,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub account: Account,
    pub mysql: Mysql,
    pub redis: Redis,
}

pub async fn load_config() -> anyhow::Result<Config> {
    let mut config = Config {
        account: Account {
            uin: 123456789,
            password: "echo -n password".to_string(),
        },
        mysql: Mysql {
            host: "127.0.0.1".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "password".to_string(),
            database: "mobot".to_string(),
        },
        redis: Redis {
            host: "127.0.0.1".to_string(),
            port: 6379,
        },
    };
    if Path::new(CONFIG_FILE_PATH).exists() {
        config = serde_yaml::from_str(&std::fs::read_to_string(CONFIG_FILE_PATH)?)?;
    } else {
        let data = serde_yaml::to_string(&config)?;
        std::fs::write(CONFIG_FILE_PATH, data)?;
    };
    if config.account.uin == 123456789 {
        println!("请修改 bot.yaml");
        exit(0);
    };
    Ok(config)
}
