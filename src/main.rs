use std::process::{Command, Child};
use std::io::Result;
use std::slice;
use ipc_comms;

fn launch_bkg_proc(name : &str, args : Option<&[&str]>) -> Result<Child> {
    let exe_path = ipc_comms::get_dir_of_curr_exe().expect("Failed to get path");
    let proc_to_launch = exe_path.join(name);
    let mut proc = Command::new(proc_to_launch);
    match args {
        Some(args) => for arg in args { proc.arg(arg); },
        _ => ()
    }
    proc.spawn()
}

fn main() {
    let pipe_name = "pipe_name";
    let proc_args = Option::Some(slice::from_ref(&pipe_name));
    let mut proc_1 = launch_bkg_proc("proc_1", proc_args).expect("Failed proc_1");
    let mut proc_2 = launch_bkg_proc("proc_2", proc_args).expect("Failed proc_1");

    println!("Awaiting child processes to exit...");
    proc_1.wait().expect("Failed waiting on proc_1");
    proc_2.wait().expect("Failed waiting on proc_2");
    println!("Both child processes exited!");
}
