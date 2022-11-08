use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod menu;
mod types;
mod ignore;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![ignore::module(), menu::module(),]);
}

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
