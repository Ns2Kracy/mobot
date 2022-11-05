use anyhow::Ok;
use proc_qq::{event, module, GroupMessageEvent, MessageContentTrait, Module};

use crate::modules::types::{ID, MENU};

static ID: ID = "RussionRoulette";
static NAME: MENU = "俄罗斯轮盘赌";

pub fn module() -> Module {
    module!(ID, NAME)
}

#[event]
async fn on_message(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if content.eq(NAME) {
        println!("{}", ID);

        Ok(true)
    } else {
        Ok(false)
    }
}

/// 随机生成一个只包含5个0和1个1的数组
/// 1 表示子弹, 如果随到 1 则开枪
/// 0 表示空枪, 如果随到 0 则显示一些奇怪的文本XD
async fn random_bullet() {}
