use crate::utils::CanReply;
use anyhow::Ok;
use lazy_static::lazy_static;
use proc_qq::re_exports::ricq_core::msg::elem::RQElem;
use proc_qq::{
	event, module, ClientTrait, GroupTrait, MemberTrait, MessageChainParseTrait,
	MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module,
};
use regex::Regex;
use std::time::Duration;
static COMMAND: &'static str = ".ban";
static NAME: &'static str = "[ .ban ] 禁言";

lazy_static! {
	static ref BAN_AT_TIME_REGEX: Regex =
		Regex::new("(\\s+)?.ban(\\s+)?([0-9]{1,5})(\\s+)?([smhd]?)(\\s+)?").unwrap();
}

pub fn module() -> Module {
	module!(COMMAND, NAME, on_message)
}

async fn not_in_group_and_reply(event: &MessageEvent) -> anyhow::Result<bool> {
	Ok(if !event.is_group_message() {
		event.reply_text("只能在群中使用").await?;
		true
	} else {
		false
	})
}

#[event]
async fn on_message(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		if not_in_group_and_reply(event).await? {
			return Ok(true);
		}
		event.reply_text(String::from(".ban 禁言时间 @p1 @p2 @p3 ...").as_str()).await?;
		return Ok(true);
	}
	if !event.is_group_message() {
		return Ok(false);
	}
	let group_message = event.as_group_message()?;
	if BAN_AT_TIME_REGEX.is_match(&content) {
		// todo 缓存?
		let group = event.must_find_group(group_message.inner.group_code).await?;
		let list = group_message.client.get_group_member_list(group.code, group.owner_uin).await?;
		let caller_member = list.must_find_member(event.from_uin()).await?;
		let bot_member = list.must_find_member(event.bot_uin().await).await?;

		if caller_member.is_member() {
			group_message.reply_text("您必须是群主或管理员才能使用").await?;
			return Ok(true);
		}
		if bot_member.is_member() {
			group_message.reply_text("机器人必须是群主或管理员才能使用").await?;
			return Ok(true);
		}
		let e = BAN_AT_TIME_REGEX.captures_iter(&content).next().unwrap();
		let mut time = e.get(3).unwrap().as_str().parse::<u64>()?;
		time = match e.get(5).unwrap().as_str() {
			"m" => time * 60,
			"h" => time * 60 * 60,
			"d" => time * 60 * 60 * 24,
			_ => time,
		};
		if time >= 60 * 60 * 24 * 30 {
			group_message.reply_text("最多禁言30天").await?;
			return Ok(true);
		}
		for x in group_message.inner.elements.clone().into_iter() {
			match x {
				RQElem::At(id) => {
					event
						.client()
						.group_mute(
							group_message.inner.group_code,
							id.target,
							Duration::from_secs(time),
						)
						.await?;
				}
				_ => {
					event.send_message_to_source("无法禁言".parse_message_chain()).await?;
				}
			}
		}
		return Ok(true);
	}
	Ok(false)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_ban() {
        
    }
}