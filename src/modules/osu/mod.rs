use proc_qq::Module;

mod api;
pub mod bind;
mod enums;
pub mod help;
pub mod info;

pub fn osu_modules() -> Vec<Module> {
	vec![help::module(), info::module(), bind::module()]
}
