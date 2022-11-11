use super::bind_url;
use crate::{
    database::mysql::get_conn,
    modules::types::{COMMAND, NAME},
};
use anyhow::Ok;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};

static COMMAND: COMMAND = ".bind";
static NAME: NAME = "[ .bind ] 绑定osu!账号";

pub fn module() -> Module {
    module!(COMMAND, NAME, bind)
}

#[event]
async fn bind(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    let conn = get_conn();
    if content.eq(COMMAND) {
        let code = event.from_uin().to_string();
        let url = bind_url(&code);
        let message = format!("请点击链接进行授权: {}", url);

        // 通过get_token获取token
        event
            .send_message_to_source(message.parse_message_chain())
            .await?;

        return Ok(true);
    }

    if !event.is_group_message() {
        return Ok(false);
    }

    Ok(false)
}
