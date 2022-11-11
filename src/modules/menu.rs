use super::{
    ban, game, ignore, osu,
    types::{COMMAND, NAME},
};
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};
use std::sync::Arc;

static COMMAND: COMMAND = ".menu";
static NAME: NAME = "[ .menu ] Mo所持有的力量";

pub fn menu_modules() -> Vec<Module> {
    let modules = vec![
        ignore::module(),
        module(),
        ban::module(),
        osu::help::module(),
        game::help::module(),
    ];
    modules
}

pub fn module() -> Module {
    module!(COMMAND, NAME, menu)
}

#[event]
async fn menu(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(COMMAND) {
        // 后续考虑使用更好看的格式并且以图片形式发送
        let mut menu = vec!["mobot menu: ".to_owned()];
        for (module_index, module) in Arc::new(menu_modules()).as_ref().iter().enumerate() {
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
