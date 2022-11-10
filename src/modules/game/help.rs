use crate::modules::{
    game::{guessing_box, roulette},
    types::{COMMAND, NAME},
};
use anyhow::Ok;
use lazy_static::lazy_static;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};
use std::sync::Arc;

static COMMAND: COMMAND = ".game";
static NAME: NAME = "[ .game ] 游戏列表";

pub fn module() -> Module {
    module!(COMMAND, NAME, game_help)
}

lazy_static! {
    pub static ref GAME_MODULES: Arc<Vec<Module>> =
        Arc::new(vec![guessing_box::module(), roulette::module()]);
}

#[event]
async fn game_help(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(COMMAND) {
        // osu! help
        let mut help = vec!["game lists: ".to_owned()];
        for (module_index, module) in GAME_MODULES.as_ref().iter().enumerate() {
            if module.name != "" {
                help.push(format!("\n {}. {}", module_index, module.name));
            }
        }
        event
            .send_message_to_source(help.join("").parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
