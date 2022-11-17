use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

const COMMAND: COMMAND = "查";
const NAME: NAME = "[ 查 ] 查@某人";

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
