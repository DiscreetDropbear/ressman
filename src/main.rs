mod project_access;
mod project_manager;
mod types;
mod gui;

use types::{Note, Project};
use std::collections::BTreeMap;

fn main() {
    let mut proj_mngr = project_manager::new_project_manager().unwrap();
	
	gui::main_loop(proj_mngr);
	/*
    // create a project
    let mut proj = Project::new("ressman", "/home/ajani/Dropbox/code/git/ressman");
    println!("{:?}", proj_acc.create_project(&mut proj));
	println!("{:?}", proj_acc.find_projects(""));
	println!("");

    // edit the projects path
    proj.path = "/home/ajani/Dropbox/code/ressman".into();
	println!("{:?}", proj_acc.update_project(&mut proj));
	println!("{:?}", proj_acc.find_projects(""));
	println!("");

	// create note for project
	let mut note = Note::new("this is the notes content.");
	println!("{:?}", proj_acc.create_note(&note, &proj));
	println!("{:?}", proj_acc.get_notes(&proj));
	println!("");

	// edit note
	note.content = String::from("this is the updated string.");
	println!("{:?}", proj_acc.update_note(&note, &proj));
	println!("{:?}", proj_acc.get_notes(&proj));
	println!("");

	// delete_note
	println!("{:?}", proj_acc.forget_note(&note, &proj));
	println!("{:?}", proj_acc.get_notes(&proj));
	println!("");

	// add options to the project
	proj.insert_option("option1", "value1");
	proj.insert_option("option2", "value2");
	proj.insert_option("option3", "value3");
	println!("{:?}", proj_acc.update_project(&mut proj));
	println!("{:?}", proj_acc.find_projects(""));

	proj.insert_option("option1", "Secondvalue1");
	proj.insert_option("option2", "Secondvalue2");
	proj.insert_option("option3", "Secondvalue3");
	println!("{:?}", proj_acc.update_project(&mut proj));
	println!("{:?}", proj_acc.find_projects(""));
	

	let note = Note::new("this is the notes content.");
	println!("{:?}", proj_acc.create_note(&note, &proj));
	println!("{:?}", proj_acc.get_notes(&proj));
	println!("");
	
	let note = Note::new("notes content.");
	println!("{:?}", proj_acc.create_note(&note, &proj));
	println!("{:?}", proj_acc.get_notes(&proj));
	println!("");
	
	println!("{:?}", proj_acc.forget_project(&proj));


	println!("{:?}", proj_acc.set_option("text-editor", "vim"));
	println!("{:?}", proj_acc.set_option("text-editor", "twovim"));

	let mut opts = BTreeMap::new();


	opts.insert(String::from("text-editor"), String::from("blooop"));
	opts.insert(String::from("one"), String::from("thredd"));
	opts.insert(String::from("two"), String::from("filasb"));

	println!("{:?}", proj_acc.set_options(opts));
	println!("{:?}", proj_acc.get_options());


	println!("{:?}", proj_acc.get_option("one"));
	println!("{:?}", proj_acc.get_option("doesn'tExist"));
	*/
}
