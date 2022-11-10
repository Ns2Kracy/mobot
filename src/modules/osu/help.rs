use crate::modules::{
    osu,
    types::{COMMAND, MENU, NAME},
};
use anyhow::Ok;
use lazy_static::lazy_static;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};
use std::sync::Arc;

static COMMAND: COMMAND = ".osuhelp";
static NAME: NAME = "[ .osuhelp ] osu! 的相关功能";
static MENU: MENU = "osu! 的相关功能";

pub fn module() -> Module {
    module!(COMMAND, NAME, osu_help)
}

lazy_static! {
    static ref OSU_MODULES: Arc<Vec<Module>> =
        Arc::new(vec![osu::help::module(), osu::info::module(),]);
}

#[event]
async fn osu_help(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(COMMAND) {
        // osu! help
        let mut help = vec!["osu! help: ".to_owned()];
        for (module_index, module) in OSU_MODULES.as_ref().iter().enumerate() {
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
