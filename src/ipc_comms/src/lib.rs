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

// @return Current path of the running binary, without the binary name
pub fn get_dir_of_curr_exe() -> Result<std::path::PathBuf> {
  let mut current_exe_path = std::env::current_exe()?;
  current_exe_path.pop();
  Ok(current_exe_path)
}

// Launches a process in the background
// @param bin_name Name of binary - must be in current crate's binary path
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

// Holds functions strictly for the IPC demo (not useful otherwise)
pub mod ipc_demo {

use std::io::Result;
use args::Args;
use getopts::Occur;
use ipc_channel::ipc::IpcSender;
use serde::{Serialize, Deserialize};

pub fn parse_args() -> Result<args::Args> {
    let mut args = Args::new("expected_ipc_args", "Process that demo's IPC");
    args.option("c", "channel-name", "Name of IPC channel to connect to", "", 
        Occur::Req, Option::None);
    let cl_args: Vec<String> = std::env::args().collect();
    args.parse(cl_args).expect("Unable to parse needed args");
    Ok(args)
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ChildData {
    pub proc_name : String,
    pub count_tuple: (u16, u16, u16)
}

pub fn run_demo_child(proc_name: &str) -> Result<i32> {
    println!("Started Demo Child - {:?}...", proc_name);
    let args = parse_args()?;
    let channel_name : String = args.value_of("channel-name").expect("Channel name blank?");
    println!("Rcvd channel name '{:?}' - {:?}", channel_name, proc_name);
    let chan_tx = IpcSender::<ChildData>::connect(channel_name).unwrap();
    let data = ChildData { proc_name: proc_name.to_string(), count_tuple: (0, 1, 2) };
    chan_tx.send(data).unwrap();
    println!("Send IPC Data - {:?}...", proc_name);
    Ok(0)
}

} // End mod ipc_demo