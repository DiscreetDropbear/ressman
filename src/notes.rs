use std::process::Command;
use std::{thread, time, fs, path};
use crate::i3;
//use crate::neovim::NvimSession;

// TODO: handle errors and tidy up code, probaly should also break this into smaller functions
// takes in a string slice for the content of a note,
// opens nvim within a terminal and loads in the contents of 
// the note
pub fn open_note(note_body: &str) -> Option<String>{

	let buf_string = note_buffer_path();
	let sock_string = vim_sock_path(); 
	
	fs::write(buf_string.clone(), note_body);

	// launch nvim 
	let nvim_child = Command::new("xterm")
		.args(&["-e", "nvim", &buf_string, "--listen", &sock_string])
		.spawn();   

	// handle spawn errors
	let mut nvim_child = match nvim_child{
		Ok(child) => child,
		Err(e) => {
			panic!("{:?}", e);
		}
	};

	// pause for 100 ms to make sure that the window has been opened
	thread::sleep(time::Duration::from_millis(80));

	// on i3 change the window to be floating in the centre
	let res = i3::fix_window_position(nvim_child.id());		
	// println!("{:?}", res);

	nvim_child.wait();

	let buf_path = path::Path::new(&buf_string);

	// get buffer file contents here
	if buf_path.exists(){
		return Some(String::from_utf8(fs::read(buf_path).unwrap()).unwrap());	
	}
	
	None
}

// find a free unix socket path for vim
fn vim_sock_path() -> String{
	let mut i = 0;
	let mut sock_string;  

	loop{
		// TODO: use env::temp_dir() instead of assuming "/tmp"
		sock_string = format!("/tmp/notes{}.sock", i);
		let sock_path = path::Path::new(&sock_string);
		
		if !sock_path.exists(){
			break;
		}
		i += 1;
	}

	sock_string
}

// find free path for tempory note buffer
fn note_buffer_path() -> String{
	// find a free buffer file
	let mut i = 0;
	let mut buf_string; 
	let mut buf_path;
	loop{

		// TODO: use env::temp_dir() instead of assuming "/tmp"
		buf_string = format!("/tmp/notesBuf{}", i);
		buf_path = path::Path::new(&buf_string);
		
		if !buf_path.exists(){
			break;
		}
		i += 1;
	}

	buf_string
}
