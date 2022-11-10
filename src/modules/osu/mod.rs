use reqwest::{Client};
use serde::{Deserialize, Serialize};


pub mod bind;
mod enums;
pub mod help;
pub mod info;

const CLIENT_ID: &str = "18702";
const CLIENT_SECRET: &str = "pZRxoxqpv7uxhmEETc70yhyWS51wnjrcVeZJjfDK";
const REDIRECT_URI: &str = "http://localhost:6500/osu/callback";
const API_URL: &str = "https://osu.ppy.sh/api/v2/";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Oauth2Token {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

pub async fn get_token(code: &str) -> anyhow::Result<Oauth2Token> {
    let client = Client::new();
    let url = "https://osu.ppy.sh/oauth/token";
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", REDIRECT_URI),
    ];
    let response = client.post(url).form(&params).send().await?;
    // 返回的是json字符串, 使用serde_json解析
    let token_string = response.text().await?;
    let token: Oauth2Token = serde_json::from_str(&token_string)?;
    Ok(token)
}

pub async fn refresh_token(refresh_token: &str) -> anyhow::Result<Oauth2Token> {
    let client = Client::new();
    let url = "https://osu.ppy.sh/oauth/token";
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ];
    let response = client.post(url).form(&params).send().await?;
    let token_string = response.text().await?;
    let token: Oauth2Token = serde_json::from_str(&token_string)?;
    Ok(token)
}
