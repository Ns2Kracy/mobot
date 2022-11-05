use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod types;
mod menu;
mod hello_module;
mod roulette;
mod ping;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![
        hello_module::module(),
        menu::module(),
        roulette::roulette::module(),
        ping::ping::module(),
    ]);
}

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
