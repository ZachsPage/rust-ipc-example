// Module - Launching / starting processes / main functions

use std::io::Result;
use std::process::{Command, Child};

// Runs a "main" function and converts any errors to an int error code 
// - Mainly for use with `std::process::exit( ret_code )`
pub fn run_and_return_code(app_main_fn: fn() -> Result<i32>) -> i32 {
    match app_main_fn() {
        Ok(_) => 0,
        Err(err) => { 
            eprintln!("App Error {:?}", err);
            1
        }
    }
}

// Returns Current path of the running binary, without the binary name
pub fn get_dir_of_curr_exe() -> Result<std::path::PathBuf> {
  let mut current_exe_path = std::env::current_exe()?;
  current_exe_path.pop();
  Ok(current_exe_path)
}

// Launches a process in the background
//  bin_name - Name of binary - must be in current crate's binary path
// Passing arguments:
// No args - `launch_bkg_proc("proc_name", Option::None)`
// Single arg - 
// `let arg = "value"; let proc_args = Option::Some(slice::from_ref(&arg));`
// Multiple args -
// `let args = ["-p", "value for option p"];`
//  `let proc_args = Option::Some(&pipe_name[..]);`
pub fn launch_bkg_proc(bin_name: &str, args : Option<&[&str]>) -> Result<Child> {
    let exe_path = get_dir_of_curr_exe().expect("Failed to get curr exe path");
    let proc_to_launch = exe_path.join(bin_name);
    let mut proc = Command::new(proc_to_launch);
    if let Some(args) = args { for arg in args { proc.arg(arg); } };
    proc.spawn()
}
