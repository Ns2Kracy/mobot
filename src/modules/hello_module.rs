pub use proc_qq::re_exports::async_trait::async_trait;
use proc_qq::re_exports::ricq::client::event::GroupMessageEvent;
use proc_qq::re_exports::tracing;
use proc_qq::{
    event, module, LoginEvent, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageEventProcess, MessageSendToSourceTrait, Module, MessageChainAppendTrait, TextEleParseTrait,
};

use crate::utils::CanReply;
static NAME: &'static str = "ping";
static MENU: &'static str = "ping";
#[event]
async fn login(event: &LoginEvent) -> anyhow::Result<bool> {
    tracing::info!("登录成功了 : {}", event.uin);
    Ok(false)
}

#[event]
async fn print(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(NAME) {
        event
            .send_message_to_source(event.make_reply_chain().await.append(MENU.parse_text()))
            .await?;
        Ok(true)
    } else if content.eq("ping") {
        event
            .send_message_to_source("pong".parse_message_chain())
            .await?;
        Ok(true)
    } else if content.eq("RC") {
        event
            .send_message_to_source("NB".parse_message_chain())
            .await?;
        Ok(true)
    } else if content.eq("EX") {
        Err(anyhow::Error::msg("Text exception"))
    } else {
        Ok(false)
    }
}

#[event]
async fn group_hello(_: &GroupMessageEvent) -> anyhow::Result<bool> {
    Ok(false)
}

struct OnMessage;

#[async_trait]
impl MessageEventProcess for OnMessage {
    async fn handle(&self, event: &MessageEvent) -> anyhow::Result<bool> {
        self.do_some(event).await?;
        Ok(true)
    }
}

impl OnMessage {
    async fn do_some(&self, _: &MessageEvent) -> anyhow::Result<()> {
        Ok(())
    }
}

async fn do_some(_event: &MessageEvent) -> anyhow::Result<()> {
    Ok(())
}

#[event]
async fn handle(event: &MessageEvent) -> anyhow::Result<bool> {
    do_some(event).await?;
    Ok(true)
}

pub fn module() -> Module {
    module!("hello", "你好", login, print, group_hello, handle,)
}
