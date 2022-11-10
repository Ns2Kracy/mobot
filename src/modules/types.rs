/// COMMAND 用作触发关键字
pub(crate) type COMMAND = &'static str;

/// NAME 当作菜单用的解释说明
pub(crate) type NAME = &'static str;

/// MENU 用于回复某个模块中的功能
/// 例如 .osuhelp
/// 回复 osu! 的相关功能
/// 仅用于少量的模块, 且确定不会增添新功能
pub(crate) type MENU = &'static str;
