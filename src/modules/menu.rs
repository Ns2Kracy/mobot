use crate::modules::all_modules;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};

use super::types::{ID, NAME};

static ID: ID = "menu";
static NAME: NAME = "菜单";

pub fn module() -> Module {
    module!(ID, NAME, menu)
}

#[event]
async fn menu(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(NAME) {
        // 后续考虑使用更好看的格式并且以图片形式发送
        let mut menu = vec!["菜单(请直接回复功能名): ".to_owned()];
        for (module_index, module) in all_modules().as_ref().iter().enumerate() {
            if module.name != "" {
                menu.push(format!("\n {}. {}", module_index, module.name));
            }
        }
        event
            .send_message_to_source(menu.join("").parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
