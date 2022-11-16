use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

const COMMAND: COMMAND = "猜拳";
const NAME: NAME = "[ 猜拳 ]";

pub fn module() -> Module {
	module!(COMMAND, NAME, guessing_box)
}

#[event]
async fn guessing_box(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		event.send_message_to_source("开发中".parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
