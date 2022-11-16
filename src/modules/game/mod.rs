use proc_qq::Module;

mod guessing_box;
pub mod help;
mod roulette;

pub fn game_modules() -> Vec<Module> {
	let modules = vec![roulette::module(), guessing_box::module()];
	modules
}
