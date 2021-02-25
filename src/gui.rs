#![allow(dead_code, unused_variables)]
use std::fmt;
use crate::types::*;
use crate::project_manager::ProjectManager;
use std::process::{Command, Stdio};
use std::io::Write;
use crate::notes;

enum GuiState{
	MainMenu,
	ProjectMenu,
	NewProject,
	ManageProject(Project),
	ManageNotes(Project),
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
			},
			GuiState::ManageProject(project) => {
				state = manage_project(&mut proj_mngr, project)?;
			},
			GuiState::ManageNotes(project) => {
				state = manage_note(&mut proj_mngr, project)?;
			}
			GuiState::Exit => {break}
		}
	}

	Ok(())
}

fn main_menu(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{
	let options = vec!["Open Project (Super+p)", "New Project (Super+n)", "Exit (Super+e)"];
	
	let res = Rofi::select_option("Main Menu", options, &[Key::SuperN, Key::SuperP, Key::SuperE])?;
	
	if let Response::Enter(idx) = res{
		if idx == 0{
			return Ok(GuiState::ProjectMenu);		
		}
		else if idx == 1{
			return Ok(GuiState::NewProject);
		}
		else{
			return Ok(GuiState::Exit);
		}
	} else if let Response::SuperN(idx) = res{
		return Ok(GuiState::NewProject);

	} else if let Response::SuperP(idx) = res{
		return Ok(GuiState::ProjectMenu);

	} else if let Response::SuperE(idx) = res{
		return Ok(GuiState::Exit);
	} else if let Response::Esc = res{
		return Ok(GuiState::Exit);
	}

	Ok(GuiState::MainMenu)
}

// TODO: make this much more robust, add ways to add projects from git using a clone link
// TODO: add abillity to categories projects for better searching
fn new_project(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{

	let options = vec!["Search for new projects", "Clone a repo", "Create a new project"];
		
	let res = Rofi::select_option("New Project", options, &[])?;

	match res{
		Response::Enter(idx) => {

			// search for new projects
			if idx == 0{ 
				let mut projects = proj_mngr.find_new_projects()?;	
				println!("{:?}", projects);	
				for project in projects.iter_mut(){
					let options = vec!["Add", "Skip"];	
					let res = Rofi::select_option(project.name.as_str(), options, &[])?;
					match res{
						Response::Enter(idx) => {
							if idx == 0{
								proj_mngr.create_project(project)?;
							}

						},
						_ => {}
					}		
				}
			}
			// clone a repo
			else if idx == 1{
				let (_, projects_dir) = proj_mngr.get_option("ProjectsDir")?;				
				let repo_url = Rofi::input("Repo Url")?;

				// run git command to clone the repo into the project dir

				// get the stdout to show to the user the output

				// if the command was successfull then  
					
			}
			// create project
			else if idx == 2{
				// prompt for the title
				// prompt for language type
				// ask if this project is temporary
				// create the directory and create the project object

				// set up the directory for specific languages  
			}
		},
		_ => {}
	}

	Ok(GuiState::ProjectMenu)	
}
fn project_menu(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error>{
	
	// select a project	
	let projects = proj_mngr.find_projects("")?;		
	let proj_names: Vec<&str> = projects.iter()
		.map(|proj| &(*proj.name))
		.collect();
	
	let res = Rofi::select_option("Project Menu", proj_names, &[Key::SuperN, Key::SuperD, Key::SuperO])?;

	match res{
		Response::Esc => {
			return Ok(GuiState::MainMenu);
		},
		Response::Enter(idx) => {
			return Ok(GuiState::ManageProject(projects[idx].clone()));
		},
		Response::SuperN(idx) => {
			return Ok(GuiState::NewProject)	
		},
		Response::SuperD(idx) => {
			return Ok(GuiState::ManageProject(projects[idx].clone()));
		},
		Response::SuperO(idx) =>{
			open_project(projects[idx].clone())
		}
		_ => {}
	}
	
	return Ok(GuiState::MainMenu);
}

fn manage_project(proj_mngr: &mut ProjectManager, project: Project) -> Result<GuiState, Error>{
	let options = vec!["Open Terminal(super+t)", "Find Note(super+n)", "Create Note(super+c)", "Delete(super+d)", "Exit(super+e)"];
		
	let mut res = Rofi::select_option("Manage Project", options, &[Key::SuperT, Key::SuperN, 
		Key::SuperC, Key::SuperD, Key::SuperE])?;

	if let Response::Enter(idx) = res {
		res = if idx == 0{
			Response::SuperT(idx)
		} else if idx == 1{
			Response::SuperN(idx)
		} else if idx == 2{
			Response::SuperC(idx)
		} else if idx == 3{
			Response::SuperD(idx)
		} else if idx == 4{ 
			Response::SuperE(idx)
		}else{
			Response::Esc
		};	
	}

	match res{
		Response::SuperT(_) => {
			open_project(project.clone())
		},
		Response::SuperN(_) => {
			return Ok(GuiState::ManageNotes(project.clone()));
		},
		Response::SuperC(_) => {
			let note = Note::new("");
			proj_mngr.create_note(&note, &project)?;

			edit_note(proj_mngr, &project, &note)?;	
		},
		Response::SuperD(_) => {

		},
		Response::SuperE(_) => {
			return Ok(GuiState::Exit);
		},
		Response::Esc => {
			return Ok(GuiState::MainMenu);	
		},
		_ => {}
	}

	Ok(GuiState::ManageProject(project))
}

fn manage_note(proj_mngr: &mut ProjectManager, project: Project) -> Result<GuiState, Error>{
	// TODO: make sure notes are in desceding order based on date	
	let notes = proj_mngr.get_notes(&project)?;
	let note_names : Vec<String>= notes.iter()
		.map(|note| note.creation_date.to_rfc2822())
		.collect();
	let note_names : Vec<&str> = note_names.iter()
		.map(|note_name| note_name.as_str())
		.collect();
	
	let res = Rofi::select_option("Find Note", note_names, &[Key::SuperE])?;

	match res{
		Response::Enter(idx) => {
			edit_note(proj_mngr, &project, &notes[idx])?;
		},
		Response::SuperE(idx) => {
			return Ok(GuiState::Exit);
		},
		_ => {}
	}
	
	Ok(GuiState::ManageNotes(project))
}

fn edit_note(proj_mngr: &mut ProjectManager, project: &Project, note: &Note) -> Result<(), Error>{
	let mut note = note.clone();	
	note.content = notes::open_note(&note.content).unwrap();	
	
	proj_mngr.update_note(&note, project)?;
	Ok(())
}

// TODO: look into saving vim states and opening straight into vim
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

#[non_exhaustive]
enum Response{
	Esc,
	Enter(usize),
	SuperA(usize),
	SuperB(usize),
	SuperC(usize),
	SuperD(usize),
	SuperE(usize),
	SuperF(usize),
	SuperG(usize),
	SuperH(usize),
	SuperI(usize),
	SuperJ(usize),
	SuperK(usize),
	SuperL(usize),
	SuperM(usize),
	SuperN(usize),
	SuperO(usize),
	SuperP(usize),
	SuperQ(usize),
	SuperR(usize),
	SuperS(usize),
	SuperT(usize),
	SuperU(usize),
	SuperV(usize),
	SuperW(usize),
	SuperX(usize),
	SuperY(usize),
	SuperZ(usize)
}

#[non_exhaustive]
#[derive(Clone)]
enum Key{
	Esc,
	Enter,
	SuperA,
	SuperB,
	SuperC,
	SuperD,
	SuperE,
	SuperF,
	SuperG,
	SuperH,
	SuperI,
	SuperJ,
	SuperK,
	SuperL,
	SuperM,
	SuperN,
	SuperO,
	SuperP,
	SuperQ,
	SuperR,
	SuperS,
	SuperT,
	SuperU,
	SuperV,
	SuperW,
	SuperX,
	SuperY,
	SuperZ
}

impl fmt::Display for Key{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self{ 
			Key::Enter => write!(f, "Enter"),
			Key::SuperA => write!(f, "Super+a"),
			Key::SuperB => write!(f, "Super+b"),
			Key::SuperC => write!(f, "Super+c"),
			Key::SuperD => write!(f, "Super+d"),
			Key::SuperE => write!(f, "Super+e"),
			Key::SuperF => write!(f, "Super+f"),
			Key::SuperG => write!(f, "Super+g"),
			Key::SuperH => write!(f, "Super+h"),
			Key::SuperI => write!(f, "Super+i"),
			Key::SuperJ => write!(f, "Super+j"),
			Key::SuperK => write!(f, "Super+k"),
			Key::SuperL => write!(f, "Super+l"),
			Key::SuperM => write!(f, "Super+m"),
			Key::SuperN => write!(f, "Super+n"),
			Key::SuperO => write!(f, "Super+o"),
			Key::SuperP => write!(f, "Super+p"),
			Key::SuperQ => write!(f, "Super+q"),
			Key::SuperR => write!(f, "Super+r"),
			Key::SuperS => write!(f, "Super+s"),
			Key::SuperT => write!(f, "Super+t"),
			Key::SuperU => write!(f, "Super+u"),
			Key::SuperV => write!(f, "Super+v"),
			Key::SuperW => write!(f, "Super+w"),
			Key::SuperX => write!(f, "Super+x"),
			Key::SuperY => write!(f, "Super+y"),
			Key::SuperZ => write!(f, "Super+z"),
			Key::Esc => write!(f, "Escape")
		}
    }
	

}

fn key_to_response(key: Key, val: usize) -> Response{
	match key{
		Key::Esc => Response::Esc,
		Key::Enter => Response::Enter(val),
		Key::SuperA => Response::SuperA(val),
		Key::SuperB => Response::SuperB(val),
		Key::SuperC => Response::SuperC(val),
		Key::SuperD => Response::SuperD(val),
		Key::SuperE => Response::SuperE(val),
		Key::SuperF => Response::SuperF(val),
		Key::SuperG => Response::SuperG(val),
		Key::SuperH => Response::SuperH(val),
		Key::SuperI => Response::SuperI(val),
		Key::SuperJ => Response::SuperJ(val),
		Key::SuperK => Response::SuperK(val),
		Key::SuperL => Response::SuperL(val),
		Key::SuperM => Response::SuperM(val),
		Key::SuperN => Response::SuperN(val),
		Key::SuperO => Response::SuperO(val),
		Key::SuperP => Response::SuperP(val),
		Key::SuperQ => Response::SuperQ(val),
		Key::SuperR => Response::SuperR(val),
		Key::SuperS => Response::SuperS(val),
		Key::SuperT => Response::SuperT(val),
		Key::SuperU => Response::SuperU(val),
		Key::SuperV => Response::SuperV(val),
		Key::SuperW => Response::SuperW(val),
		Key::SuperX => Response::SuperX(val),
		Key::SuperY => Response::SuperY(val),
		Key::SuperZ => Response::SuperZ(val)
	}
}

// TODO: go over all functions within rofi and do proper error handling
impl Rofi{

	// turns a slice of keys into a vector of strings that are valid rofi arguments
	// that set up keybindings to the given keys
	fn get_keybinding_parameters(keys: &[Key]) -> Vec<String>{
		// rofi only allows for 19 custom keybindings
		if keys.len() == 20{
			panic!("can't have more than 19 custom keybindings")
		}

		let mut params = Vec::new();	
		let mut i = 1;
		for key in keys{
			params.push(format!("-kb-custom-{}", i));
			params.push(key.to_string());
			i = i+1;
		}

		params
	}

	fn get_keybinding(ret_code: i32, index: usize, keybindings: &[Key]) -> Response{
		if ret_code == 0{
			return Response::Enter(index)
		}
		else if ret_code >= 10 && ret_code <= 29{
			let idx = ret_code - 10;
			return key_to_response(keybindings[idx as usize].clone(), index) 
		}

		Response::Esc
	}

	// return the index of the selected row 
	pub fn select_option(prompt: &str, options: Vec<&str>, keybindings: &[Key]) -> Result<Response, Error> {

		let options_arr = options
			.iter()
			.map(|s| String::from(*s).replace("\n", ""))
			.collect::<Vec<String>>()
			.join("\n");


		let args = Rofi::get_keybinding_parameters(keybindings); 
		let mut args: Vec<&str> = args.iter()
			.map(|s| s.as_str())
			.collect();
		// when rofi is in dmenu mode(using -dmenu), '-format i' means it will 
		// print the index of the selected row
		args.extend_from_slice(&["-theme", "slate", "-dmenu", "-i", "-format", "i", "-p", prompt]);

		let mut rofi_child = Command::new("rofi")
			
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
		
		match usize::from_str_radix(&stdout.trim(), 10){
			Ok(index) => Ok(Rofi::get_keybinding(return_code, index, keybindings)),
			Err(_) => Ok(Response::Esc) 
		}
	}

	// return the input of the user
	// TODO: fix up the error handling
	pub fn input(prompt: &str) -> Result<String, Error>{
		let args = vec!["-dmenu", "-format", "f", "-p", prompt];

		let rofi_child = Command::new("rofi")
			.args(&args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.unwrap();

		let output = rofi_child.wait_with_output().unwrap();
		let stdout = String::from_utf8_lossy(&output.stdout);

		Ok(stdout.trim().to_string())
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



Notion of tracked projects vs un-tracked projects
*/
