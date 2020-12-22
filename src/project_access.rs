#![allow(dead_code)]
use crate::types::{Project, Note, Error};
use rusqlite::{Connection, params, NO_PARAMS}; 
use rusqlite::ffi as sqlite;
use rusqlite::ffi::ErrorCode;
use std::fs;
use std::env::{self, VarError};
use std::path::PathBuf;
use log::error;
use std::collections::BTreeMap;

static DATA_DIR_NAME: &str = "ressman";

type Result<T, E = Error> = std::result::Result<T, E>; 

// TODO: complete the from function and convert all of the neccessary functions
impl From<rusqlite::Error> for Error{
	fn from(error: rusqlite::Error) -> Self{
		println!("{:?}", error);
		match error{
			rusqlite::Error::SqliteFailure(sqlite::Error{code: ErrorCode::ConstraintViolation, extended_code: _}, _) => {
				// constraints don't only occur when an insert or update fails due to there being another row that already exists
				// but for now we will just assume this is true to simplify the error handling 
				return Error::AlreadyExists	
			}
			_ =>  return Error::GeneralError
		}
				
	}
}



#[derive(Debug)]
pub struct ProjectAccess{
	conn: Connection 
}

// a collection of functions to access project resources.
// For each method that is in the public interface there is a private function that actually interfaces with the database, the 
// public method calls this "internal" method and then manages and log any erros and returns the required results
impl ProjectAccess{

	
	pub fn get_options(&mut self) -> Result<BTreeMap<String, String>>{
		match self.internal_get_options(){
			Ok(options) => Ok(options),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn set_options(&mut self, options: BTreeMap<String, String>) -> Result<()>{
		match self.internal_set_options(options){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn get_option(&mut self, key: &str) -> Result<(String, String)>{
		match self.internal_get_option(key){
			Ok(option) => {
				if let Some(result) = option{
					return Ok(result);
				}
				else {
					return Err(Error::NotFound);
				}
			},
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn set_option(&mut self, key: &str, value: &str) -> Result<()>{
		match self.internal_set_option(key, value){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn add_project(&mut self, project: &mut Project) -> Result<()>{
		match self.internal_add_project(project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn update_project(&mut self, project: &mut Project) -> Result<()>{
		match self.internal_update_project(project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn list_projects(&mut self) -> Result<Vec<Project>, Error>{
		match self.internal_list_projects(){
			Ok(projects) => Ok(projects),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn forget_project(&mut self, project: &Project) -> Result<()>{
		match self.internal_forget_project(project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn create_note(&mut self, note: &Note, project: &Project) -> Result<(), Error>{
		match self.internal_create_note(note, project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn update_note(&mut self, note: &Note, project: &Project) -> Result<(), Error>{
		match self.internal_update_note(note, project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}
	
	pub fn list_notes(&mut self, project: &Project) -> Result<Vec<Note>, Error>{
		match self.internal_list_notes(project){
			Ok(notes) => Ok(notes),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	pub fn forget_note(&mut self, note: &Note, project: &Project) -> Result<(), Error>{
		match self.internal_forget_note(note, project){
			Ok(_) => Ok(()),
			Err(e) => {
				error!("{:?}", e);

				return Err(Error::from(e))
			}
		}
	}

	fn internal_get_options(&mut self) -> Result<BTreeMap<String, String>, rusqlite::Error>{
		let mut options = BTreeMap::new();
		
		let mut stmt = self.conn.prepare("SELECT key, value FROM Options")?;
		let mut rows = stmt.query(params![])?;

		while let Some(row) = rows.next()?{
			let key: String = row.get(0)?;
			let value: String = row.get(1)?;
			options.insert(key, value);
		}

		Ok(options)
	}

	fn internal_set_options(&mut self, options: BTreeMap<String, String>) -> Result<(), rusqlite::Error>{

		let tx = self.conn.transaction()?;
		let mut stmt = tx.prepare("INSERT INTO Options (key, value) VALUES (?, ?)
			ON CONFLICT(key) DO UPDATE SET value=excluded.value")?;
		
		for (key, value) in options{
			stmt.execute(params![key, value])?;
		}
		stmt.finalize()?;
		tx.commit()?;

		Ok(())
	}

	fn internal_get_option(&mut self, key: &str) -> Result<Option<(String, String)>, rusqlite::Error>{
		let mut stmt = self.conn.prepare("SELECT key, value FROM Options WHERE key = ?")?;
		let mut rows = stmt.query(params![key])?;

		if let Some(row) = rows.next()?{
			let key: String = row.get(0)?;
			let value: String = row.get(1)?;
			
			return Ok(Some((key, value)));
		}	
		
		Ok(None)
	}

	fn internal_set_option(&mut self, key: &str, value: &str) -> Result<(), rusqlite::Error>{
		self.conn.execute("INSERT INTO Options (key, value) VALUES (?, ?)
			ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![key, value])?;
		Ok(())
	}

	fn internal_add_project(&mut self, project: &mut Project) -> Result<(), rusqlite::Error> {
		let tx = self.conn.transaction()?;

		tx.execute("INSERT INTO Projects (name, path) VALUES (?, ?)", 
			params![project.name, String::from(project.path.to_str().unwrap())])?;

		for (key, val) in project.options.iter_mut(){
			tx.execute("INSERT INTO ProjectOptions (project_id, key, value) VALUES ((SELECT project_id 
				FROM Projects WHERE name = ?), ?, ?)", params![project.name, *key, *val])?;
		}

		tx.commit()?;

		Ok(())
	}

	
	// TODO: complete this function 
	fn internal_update_project(&mut self, project: &mut Project) -> Result<(), rusqlite::Error>{
		let tx = self.conn.transaction()?;	
		
		// TODO: use the same process as in internal_set_option using the syntax for insert or update 
		
		// delete all of the projects options
		tx.execute("DELETE FROM ProjectOptions WHERE project_id = (SELECT project_id FROM
			Projects WHERE name = ?)", params![project.name])?;

		// insert all of the "new" project options
		for (key, val) in project.options.iter_mut(){
			tx.execute("INSERT INTO ProjectOptions (project_id, key, value) VALUES ((SELECT project_id 
				FROM Projects WHERE name = ?), ?, ?)", params![project.name, *key, *val])?;
		}
		
		// update project 
		tx.execute("UPDATE Projects SET path = ? WHERE name = ?", params![project.path.to_str().unwrap(), 
			project.name])?;

		tx.commit()?;
		Ok(())
	}

	fn internal_list_projects(&mut self) -> Result<Vec<Project>, rusqlite::Error>{
			
		// get each of the projects
		let mut stmt = self.conn.prepare("SELECT name, path FROM Projects")?;
		let mut rows = stmt.query(NO_PARAMS)?;
		let mut projects = Vec::new();
		while let Some(row) = rows.next()?{
			let name : String = row.get(0)?;
			let path : String = row.get(1)?;
			projects.push(Project::new(name.as_str(), path.as_str()))
		}  
		
		// for all of the projects get their options
		for project in projects.iter_mut(){
			let mut stmt = self.conn.prepare("SELECT key, value FROM ProjectOptions WHERE project_id = 
				(SELECT project_id FROM Projects WHERE name = ?)")?;
			let mut rows = stmt.query(params![project.name])?;

			while let Some(row) = rows.next()?{
				let key: String = row.get(0)?;
				let value: String = row.get(1)?;
				project.insert_option(key.as_str(), value.as_str());
			}
		}

		Ok(projects)
	}

	fn internal_forget_project(&mut self, project: &Project) -> Result<(), rusqlite::Error>{
		
		let tx = self.conn.transaction()?;
		
		// delete the notes associated with the project
		tx.execute("DELETE FROM Notes WHERE project_id = (SELECT project_id FROM Projects WHERE name = ?)"
			, params![project.name])?;	
		// delete the options associated with the project
		tx.execute("DELETE FROM ProjectOptions WHERE project_id = (SELECT project_id FROM Projects WHERE name = ?)"
			, params![project.name])?;	
		// delete the project
		tx.execute("DELETE FROM Projects WHERE name = ?", params![project.name])?;
		tx.commit()?;
		Ok(())
	}

	fn internal_create_note(&mut self, note: &Note, project: &Project) -> Result<(), rusqlite::Error>{
		self.conn.execute("INSERT INTO Notes (project_id, created_date, content) VALUES 
			((SELECT project_id FROM Projects WHERE name = ?), ?, ?)", 
			params![project.name, note.creation_date, note.content])?;

		Ok(())
	}

	fn internal_update_note(&mut self, note: &Note, project: &Project) -> Result<(), rusqlite::Error>{
		self.conn.execute("UPDATE Notes SET content = ? WHERE created_date = ? AND project_id = 
			(SELECT project_id FROM Projects WHERE name = ?)", params![note.content, note.creation_date, project.name])?;
		Ok(())
	}

	fn internal_list_notes(&mut self, project: &Project) -> Result<Vec<Note>, rusqlite::Error>{
		let mut stmt = self.conn.prepare("SELECT created_date, content FROM Notes WHERE project_id = 
			(SELECT project_id FROM Projects WHERE name = ?)")?;
		let mut rows = stmt.query(params![project.name])?;
		let mut notes = Vec::new();
		while let Some(row) = rows.next()?{
			notes.push(Note{creation_date: row.get(0)?, content: row.get(1)?});	
		}

		Ok(notes)
	}

	fn internal_forget_note(&mut self, note: &Note, project: &Project) -> Result<(), rusqlite::Error>{
		self.conn.execute("DELETE FROM Notes WHERE created_date = ? AND project_id = (SELECT project_id FROM Projects WHERE name = ?)", params![note.creation_date, project.name])?;
		Ok(())
	}
}

// setup the struct, connect to the db and setup the tables
pub fn setup_project_access() -> Result<ProjectAccess, Error>{
	
	match setup_app_data_dir(){
		Ok(_) => {},
		Err(e) => {
			#[cfg(test)]
			{
				panic!(format!("setup_tables failed {}", e));
			}
			
			#[allow(unreachable_code)]
			error!("{:?}", e);
			return Err(Error::InitialisationError)
		}
	}

	// get the path to the db file
	let mut db_path = match get_app_data_path(){
		Ok(path) => path,
		Err(_) => panic!("couldn't find the path to store application data")   
	};
	db_path.push("ressman.db");

	// connect to the db
	let conn = match Connection::open(db_path){
		Ok(conn) => conn,
		Err(e) => {
			#[cfg(test)]
			{
				panic!(format!("setup_tables failed {}", e));
			}
			
			#[allow(unreachable_code)]
			error!("{:?}", e);	
			return Err(Error::InitialisationError)
		}	
	};
		

	match setup_tables(&conn){
		Ok(()) => {},
		Err(e) => {
			#[cfg(test)]
			{
				panic!(format!("setup_tables failed {:?}", e));
			}
			
			#[allow(unreachable_code)]
			error!("{:?}", e);	
			return Err(Error::InitialisationError)
		}	
	}
	// setup the tables	

	Ok(ProjectAccess{conn})	
}

fn setup_tables(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error>{

	// TODO: Create an index on project name column
	conn.execute_batch("
				BEGIN;

				CREATE TABLE IF NOT EXISTS Projects (
                project_id    INTEGER PRIMARY KEY,
                name          TEXT NOT NULL UNIQUE,
                path          TEXT NOT NULL );
				
				CREATE TABLE IF NOT EXISTS Notes (
				note_id			INTEGER PRIMARY KEY,
				project_id      TEXT NOT NULL,
				created_date    DATE NOT NULL,
				content			TEXT NOT NULL,
				CONSTRAINT unq UNIQUE (project_id, created_date)
				FOREIGN KEY(project_id) REFERENCES projects(project_id));
				
				CREATE TABLE IF NOT EXISTS ProjectOptions(
                project_option_id     INTEGER PRIMARY KEY,
                project_id			TEXT NOT NULL,
				key					TEXT NOT NULL,
                value					TEXT NOT NULL,
				FOREIGN KEY(project_id) REFERENCES projects(project_id));

				CREATE TABLE IF NOT EXISTS Options(
				key			TEXT PRIMARY KEY,
                value		TEXT NOT NULL);

				COMMIT;")?;	

	Ok(())
} 

fn get_app_data_path() -> Result<PathBuf, VarError>{
	
	// TODO: This looks ugly probably re-write it in a cleaner way
	let data_path = match env::var("XDG_CONFIG_HOME"){
		Ok(val) => val,
		Err(_) => {   
				match env::var("HOME"){
					Ok(val) => val,
					Err(e) => return Err(e) 
				}
		}
	};

	let mut data_path = PathBuf::from(data_path);
	data_path.push(".config");
	data_path.push(DATA_DIR_NAME);

	Ok(data_path)
}

fn setup_app_data_dir() -> std::io::Result<()>{
	let storage_path = match get_app_data_path(){
		Ok(path) => path,
		// TODO: figure out a better way to handle this error other than just panicking
		// probably by wrapping the result types inside a variant of ProjectAccesError
		Err(_) => panic!("couldn't find the path to store application data")   
	};

	// create ressman directory inside local storage directory 
	fs::create_dir_all(storage_path)?;

	Ok(())
}


#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn initialisation(){
		setup_project_access();
	}

	#[test]
	fn get_projects(){
		
		let mut pa = match setup_project_access(){
			Ok(pa) => pa,
			Err(_) => panic!("")
		};	

		let mut proj = Project::new("project1", "path");
		proj.insert_option("op", &String::from("va"));
		proj.insert_option("op2", &String::from("val"));
		proj.insert_option("op3", &String::from("valu"));

		println!("{:?}", pa.add_project(&mut proj));
		
		let mut proj = Project::new("project2", "path");
		proj.insert_option("op2", &String::from("val"));

		pa.add_project(&mut proj);
		
		let res = pa.list_projects();
		match res{
			Ok(projects) => {
				println!("{:?}", projects);
				for project in projects{
					println!("{:?}", project);
				}
			},
			Err(_) => panic!()
		}
	}

	#[test]
	fn insert_note(){
		let mut pa = match setup_project_access(){
			Ok(pa) => pa,
			Err(_) => panic!("")
		};	

		let mut proj = Project::new("project3", "path");
		pa.add_project(&mut proj.clone());
		
		let note = Note{ creation_date: chrono::Utc::now(), content: String::from("")};  
		
		pa.create_note(&note, &proj);
	}
}
