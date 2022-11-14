/// 屏蔽一些特殊账号的消息
use crate::modules::types::{COMMAND, NAME};
use proc_qq::{
    event, module, ClientTrait, FriendMessageEvent, GroupMessageEvent, GroupTempMessageEvent,
    MessageEvent, MessageSendToSourceTrait, Module,
};

static COMMAND: COMMAND = ".ignore";
static NAME: NAME = "";

pub fn module() -> Module {
    module!(
        COMMAND,
        NAME,
        on_message,
        on_friend_message,
        on_group_message,
        on_temp_message,
    )
}

fn in_ignore_list(uin: i64) -> bool {
    let list: Vec<i64> = vec![
        2854196310, // Q群管家
        2854196312, // 表情包老铁
        2854196306, // 微软小冰
    ];
    list.contains(&uin)
}

#[event]
async fn on_message(event: &MessageEvent) -> anyhow::Result<bool> {
    if event.from_uin() == event.bot_uin().await || in_ignore_list(event.from_uin()) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[event]
async fn on_friend_message(event: &FriendMessageEvent) -> anyhow::Result<bool> {
    if event.from_uin() == event.bot_uin().await || in_ignore_list(event.from_uin()) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[event]
async fn on_group_message(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    if event.from_uin() == event.bot_uin().await || in_ignore_list(event.from_uin()) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[event]
async fn on_temp_message(event: &GroupTempMessageEvent) -> anyhow::Result<bool> {
    if event.from_uin() == event.bot_uin().await || in_ignore_list(event.from_uin()) {
        Ok(true)
    } else {
        Ok(false)
    }
}
