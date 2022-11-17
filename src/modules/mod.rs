use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod game;
mod image_lib;
mod manager;
mod osu;

pub(crate) type COMMAND = &'static str;
pub(crate) type NAME = &'static str;

lazy_static! {
	static ref MODULES: Arc<Vec<Module>> = {
		Arc::new(
			vec![
				manager::manager_modules(),
				osu::osu_modules(),
				game::game_modules(),
				image_lib::image_lib_modules(),
			]
			.into_iter()
			.map(|modules| modules.into_iter())
			.flatten()
			.collect(),
		)
	};
}

pub fn all_modules() -> Arc<Vec<Module>> {
	MODULES.clone()
}
