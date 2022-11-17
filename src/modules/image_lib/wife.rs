use crate::utils::CanReply;
use anyhow::{Context, Ok};
use proc_qq::re_exports::{bytes, reqwest};
use proc_qq::{
	event, module, MessageChainAppendTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};
use regex::Regex;

use super::no_temp_message;

static COMMAND: &'static str = "随机老婆";
static NAME: &'static str = "[ 随机老婆 ]";
static UA: &'static str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.80 Mobile Safari/537.36";

pub fn module() -> Module {
	module!(COMMAND, NAME, random_wife)
}

#[event]
async fn random_wife(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		if event.is_temp_message() {
			event.send_message_to_source(no_temp_message()).await?;
			return Ok(true);
		}
		let img = get_wife_img().await?.to_vec();
		let img = event.upload_image_to_source(img).await?;
		event.send_message_to_source(event.make_reply_chain().await.append(img)).await?;
		tracing::info!("send img to {}", event.from_uin());
		Ok(true)
	} else {
		Ok(false)
	}
}

async fn get_wife_img() -> anyhow::Result<bytes::Bytes> {
	let text = reqwest::ClientBuilder::new()
		.danger_accept_invalid_certs(true)
		.build()?
		.request(reqwest::Method::GET, "https://img.xjh.me/random_img.php")
		.header("User-Agent", UA)
		.send()
		.await?
		.error_for_status()?
		.text()
		.await?;
	let regex = Regex::new("src=\"//(.+\\.jpg)\"")?;
	let mt = regex.captures_iter(&text).next().with_context(|| "未能成功取得图片")?;
	let url = format!("https://{}", mt.get(1).unwrap().as_str());
	let buff = reqwest::ClientBuilder::new()
		.danger_accept_invalid_certs(true)
		.build()?
		.request(reqwest::Method::GET, url.as_str())
		.header("User-Agent", UA)
		.send()
		.await?
		.error_for_status()?
		.bytes()
		.await?;
	Ok(buff)
}
