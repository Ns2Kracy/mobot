use proc_qq::Module;

pub mod ban;
pub mod ignore;
pub mod menu;
pub mod mobot;

pub fn manager_modules() -> Vec<Module> {
	vec![ignore::module(), ban::module(), mobot::module(), menu::module()]
}
