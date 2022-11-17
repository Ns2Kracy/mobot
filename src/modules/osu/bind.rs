use crate::{
	database::mysql::get_conn,
	modules::{COMMAND, NAME},
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
		event.send_message_to_source("开发中".parse_message_chain()).await?;
		return Ok(true);
	}

	if !event.is_group_message() {
		return Ok(false);
	}

	Ok(false)
}
