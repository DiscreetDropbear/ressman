use crate::types::Error;
use std::process::Command;
use std::{fs, path};
use log::error;
//use crate::neovim::NvimSession;

// TODO: handle errors and tidy up code, probaly should also break this into smaller functions
// takes in a string slice for the content of a note,
// opens nvim within a terminal and loads in the contents of
// the note
pub fn open_note(note_body: &str) -> Result<String, Error>{
    let buf_string = note_buffer_path();
    let sock_string = vim_sock_path();

    if let Err(e) = fs::write(buf_string.clone(), note_body){
		error!("failed to write the note string to file: {}", e);

	}

    // launch nvim
    let nvim_child = Command::new("xterm")
        .args(&["-e", "nvim", &buf_string, "--listen", &sock_string])
        .spawn();

    // handle spawn errors
    let mut nvim_child = match nvim_child{
		Ok(child) => child,
		Err(e) => {
			return Err(Error::GeneralError(format!("spawning nvim failed with error: {}", e))); 
		}
    };

    if let Err(e) = nvim_child.wait(){
		error!("error waiting for nvim to exit: {}", e);	
	}

    let buf_path = path::Path::new(&buf_string);

    // get buffer file contents here
	// TODO: deal with unwraps here, turn them into proper error handling
    if buf_path.exists() {
        return Ok(String::from_utf8(fs::read(buf_path).unwrap()).unwrap());
    }

   Ok(String::from(""))
}

// find a free unix socket path for vim
fn vim_sock_path() -> String {
    let mut i = 0;
    let mut sock_string;

    loop {
        // TODO: use env::temp_dir() instead of assuming "/tmp"
        sock_string = format!("/tmp/notes{}.sock", i);
        let sock_path = path::Path::new(&sock_string);

        if !sock_path.exists() {
            break;
        }
        i += 1;
    }

    sock_string
}

// find free path for tempory note buffer
fn note_buffer_path() -> String {
    // find a free buffer file
    let mut i = 0;
    let mut buf_string;
    let mut buf_path;
    loop {
        // TODO: use env::temp_dir() instead of assuming "/tmp"
        buf_string = format!("/tmp/notesBuf{}", i);
        buf_path = path::Path::new(&buf_string);

        if !buf_path.exists() {
            break;
        }
        i += 1;
    }

    buf_string
}
