use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod ban;
mod bilibili;
mod game;
mod ignore;
mod menu;
mod osu;
mod random_image;
mod types;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = {
        let mut modules = vec![];
        modules.extend(menu::menu_modules());
        modules.extend(osu::osu_modules());
        modules.extend(game::game_modules());
        Arc::new(modules)
    };
}

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
