use std::process::Command;
use ipc_comms;

fn launch_proc_and_print_output(name : &str) {
    let exe_path = ipc_comms::get_dir_of_curr_exe().expect("Failed to get path");
    let proc_to_launch = exe_path.join(name);
    let proc_res = Command::new(proc_to_launch).output().expect("Failed to launch");
    let output_string = std::str::from_utf8(&proc_res.stdout).expect("Failed to get output");
    println!("{}", output_string);
}

fn main() {
    println!("Hello, world!");
    launch_proc_and_print_output("proc_1");
    launch_proc_and_print_output("proc_2");
}
