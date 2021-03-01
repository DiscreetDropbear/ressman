#![allow(dead_code, unused_variables)]
use crate::notes;
use crate::project_manager::ProjectManager;
use crate::types::*;
use crate::git;
use crate::rofi::{self, Key, Response};

use std::process::Command;
use regex::Regex;
use std::path::Path;
enum GuiState {
    ProjectMenu,
    NewProject,
    ManageProject(Project),
    ManageNotes(Project),
    Exit,
}

pub fn main_loop(mut proj_mngr: ProjectManager) -> Result<(), Error> {

    let mut state = match retrieve_last_project(&mut proj_mngr)?{
      Some(project) => {GuiState::ManageProject(project)},
      None => {GuiState::ProjectMenu}
    };  

    // find the last used project
    loop {
        match state {
            GuiState::NewProject => {
                state = new_project(&mut proj_mngr)?;
            }
            GuiState::ProjectMenu => {
                state = project_menu(&mut proj_mngr)?;
            }
            GuiState::ManageProject(project) => {
                state = manage_project(&mut proj_mngr, project)?;
            }
            GuiState::ManageNotes(project) => {
                state = manage_note(&mut proj_mngr, project)?;
            }
            GuiState::Exit => break,
        }
    }

    Ok(())
}


fn retrieve_last_project(proj_mngr: &mut ProjectManager) -> Result<Option<Project>, Error>{
    match proj_mngr.get_option("last_used_project")? {
        Some((_, proj_name)) => match proj_mngr.get_project(&proj_name)? {
            Some(proj) => Ok(Some(proj)),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

// TODO: make this much more robust, add ways to add projects from git using a clone link
// TODO: add abillity to categories projects for better searching
fn new_project(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error> {
    let options = vec![
        "Search in project directory (Super+s)",
        "Clone a git repo (Super+c)",
        "Create a new local project (Super+l)",
    ];

    let res = rofi::select_option("New Project", options, &[Key::SuperS, Key::SuperC, Key::SuperL])?;

    match res {
        Response::Enter(idx) => {
            // search for new projects
            if idx == 0 {
                let mut projects = proj_mngr.find_new_projects()?;
                println!("{:?}", projects);
                for project in projects.iter_mut() {
                    let options = vec!["Add", "Skip"];
                    let res = rofi::select_option(project.name.as_str(), options, &[])?;
                    match res {
                        Response::Enter(idx) => {
                            if idx == 0 {
                                proj_mngr.create_project(project)?;
                            }
                        }
                        _ => {}
                    }
                }
            }
            // clone a repo
            else if idx == 1 {
                
                //
                // add the new repo into the database

            // run git command to clone the repo into the project dir

            // get the stdout to show to the user the output

            // if the command was successfull then
            }
            // create project
            else if idx == 2 {
                // prompt for the title
                // prompt for language type
                // ask if this project is temporary
                // create the directory and create the project object

                // set up the directory for specific languages
            }
        }
        _ => {}
    }

    Ok(GuiState::ProjectMenu)
}

fn clone_repo(proj_mngr: &mut ProjectManager) -> Result<(), Error>{
  let repo_url = rofi::input("Repo Url")?;
  
  // get repo name
  // TODO: use appropriate error handling here
  let re = Regex::new(r"https://.*?/.*?/(.*)\.git|https://.*?/.*?/(.*)|git@.*/(.*)\.git").unwrap();
  let captures = re.captures(&repo_url).unwrap();  
  
  let mut index: usize = 0; 
  for i in 1..captures.len(){
    index = index + 1;
    if let Some(_) = captures.get(i){
      break;
    }
  }

  let repo_name = captures.get(index).unwrap().as_str();
  
  let projects_dir = match proj_mngr.get_option("ProjectsDir")? {

      Some((_, projects_dir)) => projects_dir,
      None => panic!("option not found"), //TODO: change this to an appropriate response like showing an error and aborting
  };
  let repo_dir = Path::new(&projects_dir);
  let mut repo_dir = repo_dir.to_path_buf();
  repo_dir.push(&repo_name);
  match git::clone_repo(&repo_url, repo_dir.as_path()){
    Ok(v) => {} 
    Err(e) => {println!("{:?}", e)}
  }

  Ok(())
}

fn project_menu(proj_mngr: &mut ProjectManager) -> Result<GuiState, Error> {
    // select a project
    let projects = proj_mngr.find_projects("")?;
    let proj_names: Vec<&str> = projects.iter().map(|proj| &(*proj.name)).collect();

    let res = rofi::select_option(
        "Project Menu",
        proj_names.clone(),
        &[Key::SuperE, Key::SuperN, Key::SuperD, Key::SuperO, Key::SuperP],
    )?;

    match res {
        Response::Esc => {
            return Ok(GuiState::Exit);
        }
        Response::SuperE(_) => {
          return Ok(GuiState::Exit);
        }
        Response::Enter(idx) => {
            proj_mngr.set_option("last_used_project", proj_names[idx].clone())?;
            return Ok(GuiState::ManageProject(projects[idx].clone()));
        }
        Response::SuperN(idx) => return Ok(GuiState::NewProject),
        Response::SuperD(idx) => {
            return Ok(GuiState::ManageProject(projects[idx].clone()));
        }
        Response::SuperC(_) => {
          return Ok(GuiState::NewProject);
        }
        Response::SuperO(idx) => {
            proj_mngr.set_option("last_used_project", proj_names[idx].clone())?;
            open_project(projects[idx].clone())
        }
        Response::SuperP(_) => {}

        _ => {}
    }

    return Ok(GuiState::ProjectMenu);
}

fn manage_project(proj_mngr: &mut ProjectManager, project: Project) -> Result<GuiState, Error> {
    let options = vec![
        "Open Terminal(super+t)",
        "Find Note(super+n)",
        "Create Note(super+c)",
        "Delete(super+d)",
        "Exit(super+e)",
    ];

    let mut res = rofi::select_option(
        &project.name,
        options,
        &[
            Key::SuperT,
            Key::SuperN,
            Key::SuperC,
            Key::SuperD,
            Key::SuperE,
            Key::SuperP,
        ],
    )?;

    // maps between the index of the options when enter is pressed and their
    // key-combination as to reduce code
    // duplication
    if let Response::Enter(idx) = res {
        res = if idx == 0 {
            Response::SuperT(idx)
        } else if idx == 1 {
            Response::SuperN(idx)
        } else if idx == 2 {
            Response::SuperC(idx)
        } else if idx == 3 {
            Response::SuperD(idx)
        } else if idx == 4 {
            Response::SuperE(idx)
        } else {
          return Ok(GuiState::ManageProject(project));
        };
    }

    match res {
        Response::SuperT(_) => {
            open_project(project.clone());
            return Ok(GuiState::Exit);
        }
        Response::SuperN(_) => {
            return Ok(GuiState::ManageNotes(project.clone()));
        }
        Response::SuperC(_) => {
            let note = Note::new("");
            proj_mngr.create_note(&note, &project)?;

            edit_note(proj_mngr, &project, &note)?;
        }
        Response::SuperD(_) => {}
        Response::SuperE(_) => {
            return Ok(GuiState::Exit);
        }
        Response::Esc | Response::SuperP(_)=> {
            return Ok(GuiState::ProjectMenu);
        }
        _ => {}
    }

    Ok(GuiState::ManageProject(project))
}

fn manage_note(proj_mngr: &mut ProjectManager, project: Project) -> Result<GuiState, Error> {
    // TODO: make sure notes are in desceding order based on date
    let notes = proj_mngr.get_notes(&project)?;
    let note_names: Vec<String> = notes
        .iter()
        .map(|note| note.creation_date.to_rfc2822())
        .collect();
    let note_names: Vec<&str> = note_names
        .iter()
        .map(|note_name| note_name.as_str())
        .collect();
    
    if note_names.len() == 0{
    return Ok(GuiState::ManageProject(project))
    }

    let res = rofi::select_option("Find Note", note_names, &[Key::SuperE, Key::SuperO])?;

    match res {
        Response::Enter(idx) => {
            edit_note(proj_mngr, &project, &notes[idx])?;
            return Ok(GuiState::Exit)
        }
        Response::SuperE(idx) => {
            return Ok(GuiState::Exit);
        }
        _ => {}
    }

    Ok(GuiState::ManageNotes(project))
}

fn edit_note(proj_mngr: &mut ProjectManager, project: &Project, note: &Note) -> Result<(), Error> {
    let mut note = note.clone();
    note.content = notes::open_note(&note.content).unwrap();
    proj_mngr.update_note(&note, project)?;
    Ok(())
}

// TODO: look into saving vim states and opening straight into vim
fn open_project(project: Project) {
    if project.path.exists() {
        Command::new("terminator")
            .arg(format! {"--working-directory={}", project.path.to_str().unwrap()})
            .spawn()
            .expect("Failed to start the terminal");
    } else {
        panic!("The path for the project selected doesn't exist anymore. exiting!");
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
