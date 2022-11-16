use crate::modules::manager::menu;
use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod game;
mod manager;
mod osu;

pub(crate) type COMMAND = &'static str;
pub(crate) type NAME = &'static str;

lazy_static! {
	static ref MODULES: Arc<Vec<Module>> = {
		Arc::new(
			vec![menu::menu_modules(), game::game_modules(), osu::osu_modules()]
				.into_iter()
				.flatten()
				.collect::<Vec<Module>>(),
		)
	};
}

pub fn all_modules() -> Arc<Vec<Module>> {
	MODULES.clone()
}
