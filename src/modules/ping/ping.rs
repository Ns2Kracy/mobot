use anyhow::Ok;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};

static ID: &'static str = "ping";
static NAME: &'static str = "ping";

pub fn module() -> Module {
    module!(ID, NAME)
}

#[event]
async fn on_message(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(NAME) {
        event
            .send_message_to_source("pong".parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
