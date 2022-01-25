use std::env;
use std::str;
use std::path::PathBuf;
use std::io::Result;

pub fn shared_fcn(name: &str) {
  println!("shared_fcn says hello to {}", name);
}

pub fn get_dir_of_curr_exe() -> Result<PathBuf> {
  let mut current_exe_path = env::current_exe()?;
  current_exe_path.pop();
  Ok(current_exe_path)
}

