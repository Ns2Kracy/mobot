use lazy_static::lazy_static;
use proc_qq::Module;
use std::sync::Arc;

mod group_admin;
mod types;
// 菜单模块
mod menu;
// 忽视名单
mod ignore;
// bilibili 相关 modules
mod bilibili;
// osu 相关 modules
mod osu;
// 图片相关 modules
mod game;
mod random_image;

lazy_static! {
    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![
        // 菜单模块
        menu::module(),
        // 忽视名单
        ignore::module(),
        // 俄罗斯轮盘赌
        game::russian_roulette::roulette::module(),
        // 群管理模块
        group_admin::module(),
    ]);
}

/// 后续优化为父子模块
/// 例如 osu 模块下有 info, menu, recent 等子模块
/// 由各个子模块自行注册,
/// 父模块只负责管理子模块
/// 比如osu模块中注册子模块
/// ```
/// lazy_static! {
///     static ref OSU_MODULES: Arc<Vec<Module>> = Arc::new(vec![]);
/// }
///
/// pub fn osu_module() -> Arc<Vec<Module>> {
///     ...
/// }
/// ```
/// 然后由父模块统一管理
/// ```
/// lazy_static! {
///    static ref MODULES: Arc<Vec<Module>> = Arc::new(vec![]);
/// }
///
/// pub fn module() -> Arc<Vec<Module>> {
///     let mut modules = MODULES.clone();
///     modules.extend(osu::osu_module().clone().into_iter());
/// }
/// ```
/// 避免在这里注册所有模块导致代码臃肿
pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
