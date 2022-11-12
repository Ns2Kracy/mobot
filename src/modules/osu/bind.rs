use super::{Oauth2Token, CLIENT_ID, REDIRECT_URI};
use crate::{
	database::mysql::get_conn,
	modules::types::{COMMAND, NAME},
};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};

static COMMAND: COMMAND = ".bind";
static NAME: NAME = "[ .bind ] 绑定osu!账号";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bind {
	state: String,
	token: Oauth2Token,
}

pub fn module() -> Module {
	module!(COMMAND, NAME, bind)
}

#[event]
async fn bind(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	let conn = get_conn();
	if content.eq(COMMAND) {
		let state = event.from_uin().to_string();
		let url = "https://osu.ppy.sh/oauth/authorize";
		let params = [
			("client_id", CLIENT_ID),
			("redirect_uri", REDIRECT_URI),
			("response_type", "code"),
			("scope", "public"),
			("state", &state),
		];
		let mut url = Url::parse(url).unwrap();
		url.query_pairs_mut().extend_pairs(params.iter());
		let message = format!("请点击链接进行授权: {}", url);
		event.send_message_to_source(message.parse_message_chain()).await?;
		conn.execute("INSERT INTO osu_bind (qq, state) VALUES (?, ?)", (event.from_uin(), state))
			.await?;
		return Ok(true);
	}

	if !event.is_group_message() {
		return Ok(false);
	}

	Ok(false)
}
