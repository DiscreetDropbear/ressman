mod project_access;
mod project_manager;
mod types;
mod gui;
mod notes;
mod git;
mod rofi;

fn main() {
	let proj_mngr = project_manager::new_project_manager().unwrap();
	gui::main_loop(proj_mngr);
}
