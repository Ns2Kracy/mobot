use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

use crate::modules::manager::menu;

mod game;
mod manager;
mod osu;
mod types;

lazy_static! {
	static ref MODULES: Arc<Vec<Module>> = {
		Arc::new(
			vec![menu::menu_modules(), game::game_modules(), osu::osu_modules()]
				// 使用chain将多个Vec<Module>合并为一个Vec<Module>
				.into_iter()
				.flatten()
				.collect::<Vec<Module>>(),

		)
	};
}

pub fn all_modules() -> Arc<Vec<Module>> {
	MODULES.clone()
}
