use proc_qq::Module;

pub mod guessing_box;
pub mod help;
pub mod roulette;

pub fn game_modules() -> Vec<Module> {
	vec![help::module(),roulette::module(), guessing_box::module()]
}
