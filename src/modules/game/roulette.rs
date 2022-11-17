use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

const COMMAND: COMMAND = "轮盘";
const NAME: NAME = "[ 轮盘 ] ";

pub fn module() -> Module {
	module!(COMMAND, NAME, roulette)
}

#[event]
async fn roulette(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		event.send_message_to_source("开发中".parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
