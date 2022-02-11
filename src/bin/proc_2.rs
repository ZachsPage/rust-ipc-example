use std::io::Result;
use ipc_comms;

fn run_app() -> Result<i32> {
    let args = ipc_comms::parse_demo_args()?;
    let pipe_name : String = args.value_of("pipe-name").expect("Pipe name blank?");
    println!("proc_2 rcvd pipe name {:?}", pipe_name);
    Ok(0)
}

fn main() {
    let ret_code = ipc_comms::run_and_return_code( run_app );
    std::process::exit( ret_code )
}