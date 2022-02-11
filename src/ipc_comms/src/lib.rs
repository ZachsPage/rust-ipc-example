use std::io::Result;
use std::process::{Command, Child};

use args::Args;
use getopts::Occur;

pub fn get_dir_of_curr_exe() -> Result<std::path::PathBuf> {
  let mut current_exe_path = std::env::current_exe()?;
  current_exe_path.pop();
  Ok(current_exe_path)
}

// No Argument
// - launch_bkg_proc("proc_name", Option::None)
// Single Argument
// - let arg = "value"; let proc_args = Option::Some(slice::from_ref(&arg));
// Multiple Arguments
// - let args = ["-p", "value for option p"];
//   let proc_args = Option::Some(&pipe_name[..]);
pub fn launch_bkg_proc(name : &str, args : Option<&[&str]>) -> Result<Child> {
    let exe_path = get_dir_of_curr_exe().expect("Failed to get curr exe path");
    let proc_to_launch = exe_path.join(name);
    let mut proc = Command::new(proc_to_launch);
    if let Some(args) = args { for arg in args { proc.arg(arg); } };
    proc.spawn()
}

pub fn parse_demo_args() -> Result<args::Args> {
    let mut args = Args::new("expected_ipc_args", "Process that demo's IPC");
    args.option("p", "pipe-name", "Name of IPC pipe to connect to", "", 
        Occur::Req, Option::None);
    let cl_args: Vec<String> = std::env::args().collect();
    args.parse(cl_args).expect("Unable to parse needed args");
    Ok(args)
}

pub fn run_and_return_code(app_main_fn: fn() -> Result<i32>) -> i32 {
    match app_main_fn() {
        Ok(_) => 0,
        Err(err) => { 
            eprintln!("App Error {:?}", err);
            1
        }
    }
}

