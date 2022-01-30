use ipc_comms;
use std::env;

fn main() {
    ipc_comms::shared_fcn("proc_1")
    let args: Vec<String> = env::args().collect();
    println!("Got args {:?}", args);
}