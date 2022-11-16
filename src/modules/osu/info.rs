use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

const COMMAND: COMMAND = ".info";
const NAME: NAME = "[ .info ] 查询玩家信息";

pub fn module() -> Module {
	module!(COMMAND, NAME, info)
}

#[event]
async fn info(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		event.send_message_to_source("开发中".parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
