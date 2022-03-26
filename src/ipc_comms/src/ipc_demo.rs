// Module - Strictly for the IPC demo (not useful otherwise)

use std::io::Result;
use args::Args;
use getopts::Occur;
use ipc_channel::ipc::IpcSender;
use serde::{Serialize, Deserialize};

// Demo data structure to send over IPC comms
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ChildData {
    pub proc_name : String,
    pub count_tuple: (u16, u16, u16)
}

// Parse the demo proc's arguments - used to get the IPC channel name
pub fn parse_args() -> Result<args::Args> {
    let mut args = Args::new("expected_ipc_args", "Process that demo's IPC");
    args.option("c", "channel-name", "Name of IPC channel to connect to", "", 
        Occur::Req, Option::None);
    let cl_args: Vec<String> = std::env::args().collect();
    args.parse(cl_args).expect("Unable to parse needed args");
    Ok(args)
}

// Entry function for each of the spawned demo processes
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
