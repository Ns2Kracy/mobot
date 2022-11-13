use oauth2::{
	basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "18702";
const CLIENT_SECRET: &str = "pZRxoxqpv7uxhmEETc70yhyWS51wnjrcVeZJjfDK";
const REDIRECT_URI: &str = "http://localhost:6500/osu/callback";
const AUTH_URL: &str = "https://osu.ppy.sh/oauth/authorize";
const TOKEN_URL: &str = "https://osu.ppy.sh/oauth/token";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Oauth2Token {
	access_token: String,
	token_type: String,
	expires_in: u64,
	refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bind {
	state: String,
	token: Oauth2Token,
}

pub fn get_authurl_csrf(state: &str) -> (String, String) {
	let client = BasicClient::new(
		ClientId::new(CLIENT_ID.to_string()),
		Some(ClientSecret::new(CLIENT_SECRET.to_string())),
		AuthUrl::new(AUTH_URL.to_string()).unwrap(),
		Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap()),
	)
	.set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string()).unwrap());
	let (auth_url, csrf_token) = client
		.authorize_url(|| CsrfToken::new(state.to_string()))
		.add_scope(Scope::new("public".to_string()))
		.url();
	(auth_url.to_string(), csrf_token.secret().to_string())
}
