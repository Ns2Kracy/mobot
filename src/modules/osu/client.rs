use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "18702";
const CLIENT_SECRET: &str = "pZRxoxqpv7uxhmEETc70yhyWS51wnjrcVeZJjfDK";
const REDIRECT_URI: &str = "http://localhost:6500/osu/callback";

// Oauth2 Client
pub struct Oauth2Client {
    client: Client,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Oauth2Token {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

impl Oauth2Client {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            client_id: CLIENT_ID.to_owned(),
            client_secret: CLIENT_SECRET.to_owned(),
        }
    }

    pub async fn get_token(&self, code: &str) -> anyhow::Result<Oauth2Token> {
        let url = "https://osu.ppy.sh/oauth/token";
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
        ];
        let response = self.client.post(url).form(&params).send().await?;
        // 返回的是json字符串, 使用serde_json解析
        let token_string = response.text().await?;
        let token: Oauth2Token = serde_json::from_str(&token_string)?;
        Ok(token)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> anyhow::Result<Oauth2Token> {
        let url = "https://osu.ppy.sh/oauth/token";
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        let response = self.client.post(url).form(&params).send().await?;
        let token_string = response.text().await?;
        let token: Oauth2Token = serde_json::from_str(&token_string)?;

        Ok(token)
    }
}
