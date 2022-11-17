use super::image_lib_modules;
use crate::modules::{COMMAND, NAME};
use anyhow::Ok;
use proc_qq::{
	event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
	MessageSendToSourceTrait, Module,
};
use std::sync::Arc;

static COMMAND: COMMAND = "图库";
static NAME: NAME = "[ 图库 ]";

pub fn module() -> Module {
	module!(COMMAND, NAME, image_lib_help)
}

#[event]
async fn image_lib_help(event: &MessageEvent) -> anyhow::Result<bool> {
	let content = event.message_content();
	if content.eq(COMMAND) {
		let mut help = vec!["图库: ".to_owned()];
		for (module_index, module) in Arc::new(image_lib_modules()).as_ref().iter().enumerate() {
			if module.name != "" {
				help.push(format!("\n {}. {}", module_index + 1, module.name));
			}
		}
		event.send_message_to_source(help.join("").parse_message_chain()).await?;
		tracing::info!("\n{}", help.join(""));
		Ok(true)
	} else {
		Ok(false)
	}
}

#[cfg(test)]
mod test {
	use crate::modules::image_lib::image_lib_modules;
	use std::sync::Arc;

	#[test]
	fn test_image_lib_menu() {
		let mut help = vec!["图库: ".to_owned()];
		for (module_index, module) in Arc::new(image_lib_modules()).as_ref().iter().enumerate() {
			if module.name != "" {
				help.push(format!("\n {}. {}", module_index + 1, module.name));
			}
		}
		println!("{:?}", help.join(""))
	}
}
