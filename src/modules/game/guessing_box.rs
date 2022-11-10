use anyhow::Ok;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};

use crate::modules::types::{COMMAND, NAME};
// 俄罗斯轮盘赌

const COMMAND: COMMAND = ".guessing_box";
const NAME: NAME = "[ .guessing_box ] 猜拳";

pub fn module() -> Module {
    module!(COMMAND, NAME, guessing_box)
}

#[event]
async fn guessing_box(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(COMMAND) {
        event
            .send_message_to_source("猜拳".parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
