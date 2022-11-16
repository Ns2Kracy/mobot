use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

const COMMAND: COMMAND = "轮盘";
const NAME: NAME = "[ 轮盘 ] ";

pub fn module() -> Module {
	module!(COMMAND, NAME, roulette)
}

/// 轮盘赌流程
/// 1. 群友A发起轮盘赌
/// 2. 开启线程锁, 防止多个轮盘赌同时进行
///     等一个线程结束后, 才能开启下一个线程
/// 3. 群友B参与轮盘赌, 以此类推
/// 4. 当某个群友命中权重最高的子弹时, 结束轮盘赌, 解除线程锁
///     然后能够开启另一个轮盘赌
#[event]
async fn roulette(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		event.send_message_to_source("开发中".parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
