use rusqlite::Connection; 
use std::fs;
use std::env::{self, VarError};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ProjectAccessError{
	InitialisationError(String)	
}

#[derive(Debug)]
pub struct ProjectAccess{
	conn: Connection 
}

// name of the directory where the 
static DATA_DIR_NAME: &str = "ressman";

impl ProjectAccess{
	
	 
	/*
	 * SAVE_PROJECT
	 * FIND_PROJECTS  
	 * LIST_PROJECTS
	 * FORGET_PROJECT
	 *
	 * SAVE_NOTE
	 * FIND_NOTES
	 * LIST_NOTES
	 * FORGET_NOTE
	 * 
	 */
}

// setup the struct, connect to the db and setup the tables
pub fn setup_project_access() -> Result<ProjectAccess, ProjectAccessError>{
	
	match setup_app_data_dir(){
		Ok(_) => {},
		Err(e) => {
			#[cfg(test)]
			{
				panic!(format!("setup_tables failed {}", e));
			}
			
			#[allow(unreachable_code)]
			return Err(ProjectAccessError::InitialisationError(format!("{:?}", e)))
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
			return Err(ProjectAccessError::InitialisationError(format!("{:?}", e)))
		}	
	};
		

	match setup_tables(&conn){
		Ok(()) => {},
		Err(e) => {
			#[cfg(test)]
			{
				panic!(format!("setup_tables failed {}", e));
			}
			
			#[allow(unreachable_code)]
			return Err(ProjectAccessError::InitialisationError(format!("{:?}", e)))
		}	
	}
	// setup the tables	

	Ok(ProjectAccess{conn})	
}

fn setup_tables(conn: &rusqlite::Connection) -> rusqlite::Result<()>{
	
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
				FOREIGN KEY(project_id) REFERENCES projects(project_id));
				
				CREATE TABLE IF NOT EXISTS ProjectOptions(
                project_option_id     INTEGER PRIMARY KEY,
                project_id			TEXT NOT NULL,
				key					TEXT NOT NULL,
                value					TEXT NOT NULL,
				FOREIGN KEY(project_id) REFERENCES projects(project_id));

				CREATE TABLE IF NOT EXISTS Options(
                option_id   INTEGER PRIMARY KEY,
				key			TEXT NOT NULL,
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
}
