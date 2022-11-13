use proc_qq::Module;



pub mod api;
pub mod bind;
mod enums;
pub mod help;
pub mod info;

pub fn osu_modules() -> Vec<Module> {
	let modules = vec![info::module(), bind::module()];
	modules
}
