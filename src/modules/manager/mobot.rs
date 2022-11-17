use crate::modules::{COMMAND, NAME};
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};

static COMMAND: COMMAND = "mo";
static NAME: NAME = "[ mo ]";

pub fn module() -> Module {
	module!(COMMAND, NAME, mobot)
}

#[event]
async fn mobot(event: &MessageEvent) -> anyhow::Result<bool> {
	let conent = event.message_content();
	let name = env!("CARGO_PKG_NAME");
	let author = env!("CARGO_PKG_AUTHORS");
	if conent.eq(COMMAND) {
		let message = format!("{} made with ❤ \n有任何问题请联系作者:{}", name, author);
		tracing::info!("\n{} send message {} to {}", name, message, event.from_uin());
		event.send_message_to_source(message.parse_message_chain()).await?;
		Ok(true)
	} else {
		Ok(false)
	}
}
