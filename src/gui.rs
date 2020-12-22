#![allow(dead_code, unused_variables)]
use crate::types::*;
use crate::project_manager::ProjectManager;
use std::process::{Command, Stdio};
use std::io::Write;
use chrono::{DateTime, Utc};

enum GuiState{
	MainMenu,
	ProjectMenu,
	NewProject,
	ManageProject(String),
	NotesMenu(String),
	Exit
}

pub fn main_loop(mut proj_mngr: ProjectManager) -> Result<(), Error>{
	
	let mut state = GuiState::ProjectMenu; 
	
	loop{
		match state{
			GuiState::MainMenu => {
				state = main_menu(&mut proj_mngr)?;
			},
			GuiState::NewProject => {
				state = new_project(&mut proj_mngr)?;
			},
			GuiState::ProjectMenu => {
				state = project_menu(&mut proj_mngr)?;
				break;
			},
			GuiState::ManageProject(project_name) => {
				state = manage_project(&mut proj_mngr, project_name)?;
			},
			GuiState::NotesMenu(project_name) => {
				state = notes_menu(&mut proj_mngr, project_name)?;
			},
			GuiState::Exit => {break}
		}
	}

	Ok(())
}

// make get_keybinding turn a key and opt_idx into a Key enum, add a usize component to each Key enum
// so act as the selected rofi option

fn main_menu(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{
	let options = vec!["New Project", "Open Project"];
	
	let (opt_idx, _key) = Rofi::select_option("", options)?;

	if opt_idx == Some(1){
		return Ok(GuiState::NewProject);
	}else{
		return Ok(GuiState::ProjectMenu);		
	}
}

// TODO: make this much more robust, add ways to add projects from git using a clone link
fn new_project(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{

	todo!();
}
fn project_menu(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{
	
	// select a project	
	let projects = proj_mngr.find_projects("")?;		
	let proj_names: Vec<&str> = projects.iter()
		.map(|proj| &(*proj.name))
		.collect();
	
	let (opt_idx, key) = Rofi::select_option("", proj_names)?;

	match key{
		Key::Esc => {
			println!("dfasdf");
		},
		Key::Enter => {
			println!("hello");
		},
		Key::AltN => {

		},
		Key::AltD => {

		},
		Key::AltO =>{
			if let Some(idx) = opt_idx{
				open_project(projects[idx].clone())
			}
		}
	}
	
	return Ok(GuiState::MainMenu);
}
fn manage_project(proj_mngr: &mut ProjectManager, project_name: String) -> Result<GuiState, Error>{
	todo!();
}
fn notes_menu(proj_mngr: &mut ProjectManager, project_name: String) -> Result<GuiState, Error>{
	todo!();
}

// 
fn open_project(project: Project){
	if project.path.exists(){
		Command::new("terminator")
			.current_dir(project.path)
			.spawn()
			.expect("Failed to start the terminal");
	}
	else{
		panic!("The path for the project selected doesn't exist anymore. exiting!");
	}
}




struct Rofi{}

enum Key{
	Enter,
	Esc,
	AltN,
	AltD,
	AltO
}

// TODO: go over all functions within rofi and do proper error handling
impl Rofi{
	// return the index of the selected row 
	pub fn select_option(prompt: &str, options: Vec<&str>) -> Result<(Option<usize>, Key), Error> {
		let options_arr = options
			.iter()
			.map(|s| String::from(*s).replace("\n", ""))
			.collect::<Vec<String>>()
			.join("\n");
		
		let mut args = vec!["-kb-custom-1", "Alt+n", "-kb-custom-2", "Alt+d", "-kb-custom-3", 
			"Alt+o", "-kb-custom-4", "Alt+m", "-dmenu", "-i", "-format", "i", "-p"];
		args.push(prompt);

		let mut rofi_child = Command::new("rofi")
			// when rofi is in dmenu mode(using -dmenu), '-format i' means it will 
			// print the index of the selected row
			.args(&args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.unwrap();

		// send options to rofi too display.
		let stdin = rofi_child.stdin.as_mut().unwrap();
		match stdin.write_all(options_arr.as_bytes()){
			Ok(_) =>{},
			Err(_) => return Err(Error::GeneralError)
		}

		let output = rofi_child.wait_with_output().unwrap();
		let stdout = String::from_utf8_lossy(&output.stdout);

		let return_code = match output.status.code(){
				Some(code) => code,
				None => -1
			};
		
		let key_binding = Rofi::get_keybinding(return_code);

		match usize::from_str_radix(&stdout.trim(), 10){
			Ok(index) => Ok((Some(index), key_binding)),
			Err(_) => Ok((None, key_binding)) 
		}
	}

	// return the input of the user
	pub fn input(prompt: &str) -> Result<String, Error>{
		let mut args = vec!("-dmenu", "-format", "f", "-p");
		args.push(prompt);

		let rofi_child = Command::new("rofi")
			// when rofi is in dmenu mode(using -dmenu), -format i means it will print the index of the selected
			// row
			.args(&args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.unwrap();

		let output = rofi_child.wait_with_output().unwrap();
		let stdout = String::from_utf8_lossy(&output.stdout);

		Ok(stdout.trim().to_string())
	}

	pub fn get_keybinding(code: i32) -> Key{
		return if code == 0{
			Key::Enter
		}
		else if code == 10{
			Key::AltN
		}
		else if code == 11{
			Key::AltD
		}
		else if code == 12{
			Key::AltO
		}
		else{
			Key::Esc
		};
	}


}

// mapping between menu state and function
// main_menu -> 

// maps between the output of a menu and the input of a function
// the functions must return a new menu state and optionally some data

/*
depending on command line arguments open straight into project selection or main menu etc
prompt to open a particular project if it was the last project used and it has been under an hour

menu options brainstorm

main menu:
	Add new project,
	Open Project

Project menu:
	Terminal
	Notes
	Manage Project

Manage Project:
	Delete Project
	Edit Project

Notes Menu:
	Create Note,
	Select Note,

*/
