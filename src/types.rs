#![allow(dead_code)]
use std::path::PathBuf;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

//TODO: move structs and impls to another more reasonable location
pub struct Note{
	pub creation_date: DateTime<Utc>,
	pub content: String
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
	pub fn insert_option(&mut self, key: &str, value: String) -> Option<String>{
		
		self.options.insert(String::from(key), value)
	}

	// TODO: probably return a wrapper iterator around the btree_map::Iter<> to hide the implementation details
	pub fn option_iter(&self) -> std::collections::btree_map::Iter<String, String >{
		self.options.iter()
	} 
}
