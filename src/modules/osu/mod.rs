use proc_qq::Module;

mod api;
mod bind;
mod enums;
pub mod help;
mod info;

pub fn osu_modules() -> Vec<Module> {
	let modules = vec![info::module(), bind::module()];
	modules
}
