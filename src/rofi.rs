use crate::types::*;

use std::fmt;
use std::process::{Command, Stdio};
use std::io::Write;

#[non_exhaustive]
pub enum Response {
  Esc,
  Enter(usize),
  SuperA(usize),
  SuperB(usize),
  SuperC(usize),
  SuperD(usize),
  SuperE(usize),
  SuperF(usize),
  SuperG(usize),
  SuperH(usize),
  SuperI(usize),
  SuperJ(usize),
  SuperK(usize),
  SuperL(usize),
  SuperM(usize),
  SuperN(usize),
  SuperO(usize),
  SuperP(usize),
  SuperQ(usize),
  SuperR(usize),
  SuperS(usize),
  SuperT(usize),
  SuperU(usize),
  SuperV(usize),
  SuperW(usize),
  SuperX(usize),
  SuperY(usize),
  SuperZ(usize),
}

#[allow(dead_code)]
#[non_exhaustive]
#[derive(Clone)]
pub enum Key {
  Esc,
  Enter,
  SuperA,
  SuperB,
  SuperC,
  SuperD,
  SuperE,
  SuperF,
  SuperG,
  SuperH,
  SuperI,
  SuperJ,
  SuperK,
  SuperL,
  SuperM,
  SuperN,
  SuperO,
  SuperP,
  SuperQ,
  SuperR,
  SuperS,
  SuperT,
  SuperU,
  SuperV,
  SuperW,
  SuperX,
  SuperY,
  SuperZ,
}

impl fmt::Display for Key {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Key::Enter => write!(f, "Enter"),
      Key::SuperA => write!(f, "Super+a"),
      Key::SuperB => write!(f, "Super+b"),
      Key::SuperC => write!(f, "Super+c"),
      Key::SuperD => write!(f, "Super+d"),
      Key::SuperE => write!(f, "Super+e"),
      Key::SuperF => write!(f, "Super+f"),
      Key::SuperG => write!(f, "Super+g"),
      Key::SuperH => write!(f, "Super+h"),
      Key::SuperI => write!(f, "Super+i"),
      Key::SuperJ => write!(f, "Super+j"),
      Key::SuperK => write!(f, "Super+k"),
      Key::SuperL => write!(f, "Super+l"),
      Key::SuperM => write!(f, "Super+m"),
      Key::SuperN => write!(f, "Super+n"),
      Key::SuperO => write!(f, "Super+o"),
      Key::SuperP => write!(f, "Super+p"),
      Key::SuperQ => write!(f, "Super+q"),
      Key::SuperR => write!(f, "Super+r"),
      Key::SuperS => write!(f, "Super+s"),
      Key::SuperT => write!(f, "Super+t"),
      Key::SuperU => write!(f, "Super+u"),
      Key::SuperV => write!(f, "Super+v"),
      Key::SuperW => write!(f, "Super+w"),
      Key::SuperX => write!(f, "Super+x"),
      Key::SuperY => write!(f, "Super+y"),
      Key::SuperZ => write!(f, "Super+z"),
      Key::Esc => write!(f, "Escape"),
    }
  }
}

fn key_to_response(key: Key, val: usize) -> Response {
  match key {
    Key::Esc => Response::Esc,
    Key::Enter => Response::Enter(val),
    Key::SuperA => Response::SuperA(val),
    Key::SuperB => Response::SuperB(val),
    Key::SuperC => Response::SuperC(val),
    Key::SuperD => Response::SuperD(val),
    Key::SuperE => Response::SuperE(val),
    Key::SuperF => Response::SuperF(val),
    Key::SuperG => Response::SuperG(val),
    Key::SuperH => Response::SuperH(val),
    Key::SuperI => Response::SuperI(val),
    Key::SuperJ => Response::SuperJ(val),
    Key::SuperK => Response::SuperK(val),
    Key::SuperL => Response::SuperL(val),
    Key::SuperM => Response::SuperM(val),
    Key::SuperN => Response::SuperN(val),
    Key::SuperO => Response::SuperO(val),
    Key::SuperP => Response::SuperP(val),
    Key::SuperQ => Response::SuperQ(val),
    Key::SuperR => Response::SuperR(val),
    Key::SuperS => Response::SuperS(val),
    Key::SuperT => Response::SuperT(val),
    Key::SuperU => Response::SuperU(val),
    Key::SuperV => Response::SuperV(val),
    Key::SuperW => Response::SuperW(val),
    Key::SuperX => Response::SuperX(val),
    Key::SuperY => Response::SuperY(val),
    Key::SuperZ => Response::SuperZ(val),
  }
}


// TODO: go over all functions within rofi and do proper error handling
// turns a slice of keys into a vector of strings that are valid rofi arguments
// that set up keybindings to the given keys
fn get_keybinding_parameters(keys: &[Key]) -> Vec<String> {
  // rofi only allows for 19 custom keybindings
  if keys.len() == 20 {
    panic!("can't have more than 19 custom keybindings")
  }

  let mut params = Vec::new();
  let mut i = 1;
  for key in keys {
    params.push(format!("-kb-custom-{}", i));
    params.push(key.to_string());
    i = i + 1;
  }

  params
}

fn get_keybinding(ret_code: i32, index: usize, keybindings: &[Key]) -> Response {
  if ret_code == 0 {
    return Response::Enter(index);
  } else if ret_code >= 10 && ret_code <= 29 {
    let idx = ret_code - 10;
    return key_to_response(keybindings[idx as usize].clone(), index);
  }

  Response::Esc
}

// return the index of the selected row
pub fn select_option(
  prompt: &str,
  options: Vec<&str>,
  keybindings: &[Key],
) -> Result<Response, Error> {
  let options_arr = options
    .iter()
    .map(|s| String::from(*s).replace("\n", ""))
    .collect::<Vec<String>>()
    .join("\n");

    let args = get_keybinding_parameters(keybindings);
    let mut args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    // when rofi is in dmenu mode(using -dmenu), '-format i' means it will
    // print the index of the selected row
    args.extend_from_slice(&[
      "-theme", "slate", "-dmenu", "-i", "-format", "i", "-p", prompt,
    ]);

    let mut rofi_child = Command::new("rofi")
      .args(&args)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    // send options to rofi to display.
    let stdin = rofi_child.stdin.as_mut().unwrap();
    match stdin.write_all(options_arr.as_bytes()) {
      Ok(_) => {}
      Err(_) => return Err(Error::GeneralError),
    }

    let output = rofi_child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    let return_code = match output.status.code() {
      Some(code) => code,
      None => -1,
    };

    match usize::from_str_radix(&stdout.trim(), 10) {
      Ok(index) => Ok(get_keybinding(return_code, index, keybindings)),
      Err(_) => Ok(Response::Esc),
    }
}

// return the input of the user
// TODO: fix up the error handling
pub fn input(prompt: &str) -> Result<String, Error> {
  let args = vec!["-dmenu", "-format", "f", "-p", prompt];

  let rofi_child = Command::new("rofi")
    .args(&args)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let output = rofi_child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.trim().to_string())
}
