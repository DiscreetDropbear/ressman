#![allow(dead_code)]
use std::path::PathBuf;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

// created this error enum to be able to return errors that don't depend on the 
// internal implementation of this module
//TODO: look into implementing the Error trait
//TODO: move this into the types module to be used as a project wide error
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error{
	InitialisationError,
	AlreadyExists,
	GeneralError,
	NotFound
}

//TODO: move structs and impls to another more reasonable location
#[derive(Debug)]
pub struct Note{
	pub creation_date: DateTime<Utc>,
	pub content: String
}

impl Note{
	pub fn new(content: &str) -> Self{
		Note{
			content: String::from(content),
			creation_date: Utc::now()
		}		
	}
}

#[derive(Debug, Clone)]
pub struct Project{
	pub name: String,
	pub path: PathBuf,
	pub options: BTreeMap<String, String>
}


impl Project{
	pub fn new(name: &str, path: &str) -> Self{
		Project{
			name: name.into(),
			path: path.into(),
			options: BTreeMap::new()
		}
	}

	pub fn get_option(&self, key: &str) -> Option<&String>{
		self.options.get(key)
	}

	// 
	pub fn insert_option(&mut self, key: &str, value: &str) -> Option<String>{
		
		self.options.insert(String::from(key), String::from(value))
	}

	// TODO: probably return a wrapper iterator around the btree_map::Iter<> to hide the implementation details
	pub fn option_iter(&self) -> std::collections::btree_map::Iter<String, String >{
		self.options.iter()
	} 
}
