#![allow(dead_code, unused_variables)]
use crate::project_access::{self, ProjectAccess};
use crate::types::{Note, Project, Error};
use std::collections::BTreeMap;

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

    pub fn find_projects(&mut self, fuzzy_match: &str) -> Result<Vec<Project>, Error> {

        let projects = self.proj_acc.list_projects()?;

        // get results of fuzzy search here

        Ok(projects)
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
