// return the index of the selected row 
pub fn select_option(prompt: &str, options: Vec<String>) -> Result<(usize, i32), ()> {
	let options_arr = options
		.iter()
		.map(|s| s.replace("\n", ""))
		.collect::<Vec<String>>()
		.join("\n");
	
	let mut args = vec!["-kb-custom-1", "Alt+n", "-kb-custom-2", "Alt+d", "-kb-custom-3", "Alt+e", "-dmenu", "-i", "-only-match", "-format", "i", "-p"];
	args.push(prompt);

	let mut rofi_child = Command::new("rofi")
		// when rofi is in dmenu mode(using -dmenu), '-format i' means it will 
		// print the index of the selected row
		.args(&args)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();

	// send options to rofi too display.
	let stdin = rofi_child.stdin.as_mut().unwrap();
	match stdin.write_all(options_arr.as_bytes()){
		Ok(_) =>{},
		Err(_) => return Err(())
	}

	let output = rofi_child.wait_with_output().unwrap();
	let stdout = String::from_utf8_lossy(&output.stdout);

	let return_code = match output.status.code(){
			Some(code) => code,
			None => -1
		};

	match usize::from_str_radix(&stdout.trim(), 10){
		Ok(index) => Ok((index, return_code)),
		Err(_) => Err(())
	}
}

// return the input of the user
pub fn input(prompt: &str) -> Result<String, ()>{
	let mut args = vec!("-dmenu", "-format", "f", "-p");
	args.push(prompt);

	let rofi_child = Command::new("rofi")
		// when rofi is in dmenu mode(using -dmenu), -format i means it will print the index of the selected
		// row
		.args(&args)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();

	let output = rofi_child.wait_with_output().unwrap();
	let stdout = String::from_utf8_lossy(&output.stdout);

	Ok(stdout.trim().to_string())
}

pub fn custom_keybinding_index(returnCode: i32) -> Option<u32>{
	if returnCode >= 10 || returnCode <= 29 {
		return Some((returnCode - 10) as u32)
	}	

	None
}

