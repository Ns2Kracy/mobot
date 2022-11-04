use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod menu;
mod ignore;
mod hello_module;
mod query;
mod osu;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![
        hello_module::module(),
        ignore::module(),
        query::image_lib::module(),
        menu::module(),
    ]);
}

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
