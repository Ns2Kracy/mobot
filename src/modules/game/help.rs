use super::game_modules;
use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};
use std::sync::Arc;

static COMMAND: COMMAND = "玩游戏";
static NAME: NAME = "[ 玩游戏 ]";

pub fn module() -> Module {
	module!(COMMAND, NAME, game_help)
}

#[event]
async fn game_help(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		let mut help = vec!["game list: ".to_owned()];
		for (module_index, module) in Arc::new(game_modules()).as_ref().iter().enumerate() {
			if module.name != "" {
				help.push(format!("\n {}. {}", module_index + 1, module.name));
			}
		}
		event.send_message_to_source(help.join("").parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
