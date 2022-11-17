use proc_qq::{
	re_exports::ricq_core::msg::MessageChain, MessageChainAppendTrait, Module, TextEleParseTrait,
};

pub mod help;
pub mod wallpaper;
pub mod wife;

fn no_temp_message() -> MessageChain {
	MessageChain::default().append("临时会话不能使用此功能奥".parse_text())
}

pub fn image_lib_modules() -> Vec<Module> {
	vec![help::module(),wife::module(), wallpaper::module()]
}
