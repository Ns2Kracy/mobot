use proc_qq::{event, MessageEvent, Module, module, MessageContentTrait, MessageSendToSourceTrait, MessageChainParseTrait};

use crate::modules::types::{COMMAND, NAME};



static COMMAND: COMMAND = ".bind";
static NAME: NAME = "[ .bind ] 绑定osu!账号";

pub fn module() -> Module {
    module!(COMMAND, NAME, bind)
}

#[event]
async fn bind(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(COMMAND) {
        // 通过get_token获取token
        event
            .send_message_to_source("请点击神秘链接, 绑定osu!账号".parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
