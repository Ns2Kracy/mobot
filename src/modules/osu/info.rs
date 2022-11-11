use crate::modules::types::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{event, module, MessageContentTrait, MessageEvent, Module};

const COMMAND: COMMAND = ".info";
const NAME: NAME = "[ .info ] 查询玩家信息";

pub fn module() -> Module {
    module!(COMMAND, NAME, info)
}

#[event]
async fn info(event: &MessageEvent) -> anyhow::Result<bool> {
    let _content = event.message_content();
    Ok(true)
}
