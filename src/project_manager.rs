#![allow(dead_code, unused_variables)]
use log::error;
use crate::project_access::{self, ProjectAccess};
use crate::types::{Note, Project, Error};
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};

pub struct ProjectManager {
    proj_acc: ProjectAccess,
}

pub fn new_project_manager() -> Result<ProjectManager, Error>{
	let proj_acc = project_access::setup_project_access()?;

	Ok(ProjectManager{
		proj_acc
	})
}

impl ProjectManager {

	pub fn get_options(&mut self) -> Result<BTreeMap<String, String>, Error>{
		Ok(self.proj_acc.get_options()?)
	}	

	pub fn set_options(&mut self, options: BTreeMap<String, String>) -> Result<(), Error>{

		Ok(self.proj_acc.set_options(options)?)
	}

	pub fn get_option(&mut self, key: &str) -> Result<(String, String), Error>{

		Ok(self.proj_acc.get_option(key)?)
	}

	pub fn set_option(&mut self, key: &str, value: &str) -> Result<(), Error>{

		Ok(self.proj_acc.set_option(key, value)?)
	}

	pub fn find_new_projects(&mut self) -> Result<Vec<Project>, Error>{
		let projects = self.proj_acc.list_projects()?;
		let (_, projects_dir)= self.proj_acc.get_option("ProjectsDir")?;				
			
		let mut dirs: HashMap<String, bool> = HashMap::new();
		// get all folders within project_dir
		let path = Path::new(&projects_dir);	
		let iter = match path.read_dir(){
			Ok(iter) => iter,
			Err(e) => {
				error!("Error reading directory: {}", e);
				return Err(Error::GeneralError);
			}

		};

		for entry in iter{
			let entry = match entry{
				Ok(entry) => entry,
				Err(e) => {
					error!("Error reading directory: {}", e);
					return Err(Error::GeneralError);
				}
			};

			if entry.path().is_dir(){
				dirs.insert(entry.file_name().into_string().unwrap(), true);
			}
		}  
		
		// find entries that are in dirs but not in projects
		for project in projects{
			dirs.remove(&project.name);
		}

		// since we removed all of the conflicting projects now all thats left in dirs are new
		// projects
		let mut new_projects = Vec::new();	
		for (proj_name, _) in dirs.iter(){
			let mut path = PathBuf::new();
				path.push(projects_dir.clone());
				path.push(proj_name);
			new_projects.push(Project::new(proj_name, path.as_path().to_str().unwrap()));
		}	
	
		Ok(new_projects)
	}


    pub fn find_projects(&mut self, fuzzy_match: &str) -> Result<Vec<Project>, Error> {

        let projects = self.proj_acc.list_projects()?;

        // get results of fuzzy search here

        Ok(projects)
    }

	pub fn get_project(&mut self, project_name: &str) -> Result<Option<Project>, Error> {
		let proj = self.proj_acc.get_project(project_name)?;

		Ok(proj)
	}

    pub fn create_project(&mut self, project: &mut Project) -> Result<(), Error> {

        self.proj_acc.add_project(project)?;

        Ok(())
    }

    pub fn update_project(&mut self, project: &mut Project) -> Result<(), Error> {

        self.proj_acc.update_project(project)?;

        Ok(())
    }

	pub fn forget_project(&mut self, project: &Project) -> Result<(), Error>{
		
        self.proj_acc.forget_project(project)?;

		Ok(())
	}

    pub fn get_notes(&mut self, project: &Project) -> Result<Vec<Note>, Error> {

        let notes = self.proj_acc.list_notes(project)?;

        Ok(notes)
    }

    pub fn create_note(&mut self, note: &Note, project: &Project) -> Result<(), Error> {

        self.proj_acc.create_note(note, project)?;

        Ok(())
    }

    pub fn update_note(&mut self, note: &Note, project: &Project) -> Result<(), Error> {

        self.proj_acc.update_note(note, project)?;

        Ok(())
    }

    pub fn forget_note(&mut self, note: &Note, project: &Project) -> Result<(), Error> {

        self.proj_acc.forget_note(note, project)?;

        Ok(())
    }
}
