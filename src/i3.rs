use i3ipc::I3Connection;
use std::process::Command;

pub fn fix_window_position(pid: u32) -> Result<(), ()>{
	
	let x_window_id = match get_x_window_id(pid){
		Ok(id) => id,
		Err(e) => {
			println!("{:?}", e);
			return Err(());
		}
	};
	println!("x_window_id = {}", x_window_id);

	// establish a connection to i3 over a unix socket
	let mut connection = I3Connection::connect().unwrap();

	// fullscreen the focused window
	let float_cmd = format!("[id=\"{}\"] floating enable", x_window_id);
	connection.run_command(&float_cmd).unwrap();

	let directions = vec!["left", "right", "up", "down"];	
	let mut resize_cmd: String;
	for direction in directions{
		resize_cmd = format!("[id=\"{}\"] resize grow {} 125", x_window_id, direction);
		connection.run_command(&resize_cmd).unwrap();
	}
	
	Ok(())
}

pub fn get_x_window_id(pid: u32) -> Result<String, ()>{

	let pid = pid.to_string();

	let output = Command::new("xdotool")
		.args(&["search", "--pid", &pid])
		.output();

	match output{
		Ok(output) => {
			Ok(String::from_utf8(output.stdout).unwrap().trim().to_string())
		},
		Err(e) => {
			println!("{:?}", e);

			Err(())
		} 
	}
}
