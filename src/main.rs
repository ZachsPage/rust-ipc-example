use std::io::Result;
use ipc_comms;

fn run_app() -> Result<i32> {
    let pipe_name = ["-p", "ipc_pipe"];
    let proc_args = Option::Some(&pipe_name[..]);
    let mut proc_1 = ipc_comms::launch_bkg_proc("proc_1", proc_args)?;
    let mut proc_2 = ipc_comms::launch_bkg_proc("proc_2", proc_args)?;

    println!("Awaiting child processes to exit...");
    proc_1.wait().expect("Failed waiting on proc_1");
    proc_2.wait().expect("Failed waiting on proc_2");
    println!("Both child processes exited!");
    Ok(0)
}

fn main() {
    let ret_code = ipc_comms::run_and_return_code( run_app );
    std::process::exit( ret_code )
}
