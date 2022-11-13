use super::api;
use crate::{
	database::mysql::get_conn,
	modules::types::{COMMAND, NAME},
};
use anyhow::Ok;
use mysql::PooledConn;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

static COMMAND: COMMAND = ".bind";
static NAME: NAME = "[ .bind ] 绑定osu!账号";

pub fn module() -> Module {
	module!(COMMAND, NAME, bind)
}

#[event]
async fn bind(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	let _conn: PooledConn = get_conn().await;
	if content.eq(COMMAND) {
		let call_id = event.from_uin().to_string();
		// 获取auth_url, get_authurl_
		let (_auth_url, _) = api::get_authurl_csrf(call_id.as_str());
		let message = format!("请点击链接进行授权: ");
		event.send_message_to_source(message.parse_message_chain()).await?;
		// 使用conn将state存入数据库
		return Ok(true);
	}

	if !event.is_group_message() {
		return Ok(false);
	}

	Ok(false)
}
