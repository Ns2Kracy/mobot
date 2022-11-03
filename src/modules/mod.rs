use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

pub mod hello_module;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![
        hello_module::module(),
    ]);
}

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}