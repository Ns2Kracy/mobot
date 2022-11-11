use proc_qq::Module;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub mod bind;
mod enums;
pub mod help;
pub mod info;

const CLIENT_ID: &str = "18702";
const CLIENT_SECRET: &str = "pZRxoxqpv7uxhmEETc70yhyWS51wnjrcVeZJjfDK";
const REDIRECT_URI: &str = "http://localhost:6500/osu/callback";
const API_URL: &str = "https://osu.ppy.sh/api/v2/";

pub fn osu_modules() -> Vec<Module> {
    let modules = vec![info::module(), bind::module()];
    modules
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Oauth2Token {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

// pub async fn auth_url(code: &str) -> anyhow::Result<Oauth2Token> {
//     let client = Client::new();
//     let url = "https://osu.ppy.sh/oauth/authorize";
//     let params = [
//         ("client_id", CLIENT_ID),
//         ("client_secret", CLIENT_SECRET),
//         ("grant_type", "authorization_code"),
//         ("code", code),
//         ("redirect_uri", REDIRECT_URI),
//     ];
//     let response = client.post(url).form(&params).send().await?;
//     // 返回的是json字符串, 使用serde_json解析
//     let token_string = response.text().await?;
//     let token: Oauth2Token = serde_json::from_str(&token_string)?;
//     Ok(token)
// }

// pub async fn refresh_token(refresh_token: &str) -> anyhow::Result<Oauth2Token> {
//     let client = Client::new();
//     let url = "https://osu.ppy.sh/oauth/token";
//     let params = [
//         ("client_id", CLIENT_ID),
//         ("client_secret", CLIENT_SECRET),
//         ("grant_type", "refresh_token"),
//         ("refresh_token", refresh_token),
//     ];
//     let response = client.post(url).form(&params).send().await?;
//     let token_string = response.text().await?;
//     let token: Oauth2Token = serde_json::from_str(&token_string)?;
//     Ok(token)
// }

/// bind 流程为
/// 1. 用户发送 .bind 指令
/// 2. 机器人发送一个链接, 用户点击链接, 跳转到osu!的授权页面
/// 3. 用户授权后, osu!会重定向到我们的服务器, 这时候我们就可以获取到code
/// 4. 我们使用code去获取token
/// 4. 使用token中的access_token, 获取用户的信息
/// 5. 每次使用token时, 都需要检查token是否过期, 如果过期, 则使用refresh_token刷新token
/// 6. 将用户的信息qq号, osu!id, token存入数据库
pub fn bind_url(state: &str) -> String {
    let url = "https://osu.ppy.sh/oauth/authorize";
    let params = [
        ("client_id", CLIENT_ID),
        ("redirect_uri", REDIRECT_URI),
        ("response_type", "code"),
        ("scope", "public"),
        ("state", state),
    ];
    let mut url = Url::parse(url).unwrap();
    url.query_pairs_mut().extend_pairs(params.iter());
    url.to_string()
}

pub async fn exchange_code(code: &str) -> anyhow::Result<Oauth2Token> {
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
    let token_string = response.text().await?;
    let token: Oauth2Token = serde_json::from_str(&token_string)?;
    Ok(token)
}

pub async fn get_token(auth_url: &str) -> anyhow::Result<Oauth2Token> {
    let client = Client::new();
    let url = "https://osu.ppy.sh/oauth/token";
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("grant_type", "authorization_code"),
        ("code", auth_url),
        ("redirect_uri", REDIRECT_URI),
    ];
    let response = client.post(url).form(&params).send().await?;
    let token_string = response.text().await?;
    let token: Oauth2Token = serde_json::from_str(&token_string)?;
    Ok(token)
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_auth_url() {
        let url = super::bind_url("2220496937");
        println!("{}", url);
    }
}
