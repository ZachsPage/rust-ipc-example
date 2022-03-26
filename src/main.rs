use std::io::Result;
use std::process::Child;
use ipc_comms;
use ipc_comms::ipc_demo::ChildData;
use ipc_channel::ipc;

type DemoIpcServer = ipc::IpcOneShotServer::<ChildData>;

fn create_proc_n_server(proc_name : &str) -> (Child, DemoIpcServer) {
    let (server, channel_name) = DemoIpcServer::new().unwrap();
    let channel_arg = ["-c", &channel_name];
    let proc_args = Option::Some(&channel_arg[..]);
    let proc = ipc_comms::launch_bkg_proc(proc_name, proc_args).unwrap();
    (proc, server)
}

fn rcv_and_verify_msg(serv: DemoIpcServer, expected_data: &ChildData) -> bool {
    let (_, rcvd_data) = serv.accept().unwrap();
    let equal = &rcvd_data == expected_data;
    if equal { println!("Valid Msg {:?}", rcvd_data); }
    else { println!("Invalid Msg:\n {:?} vs\n {:?}", expected_data, rcvd_data); }
    equal
}

fn run_app() -> Result<i32> {
    println!("Launching child processes...");
    let (mut proc_1, serv_1) = create_proc_n_server("proc_1");
    let (mut proc_2, serv_2) = create_proc_n_server("proc_2");

    println!("Awaiting child process messages...");
    // Set data to the expected values before receiving to verify content
    let mut expected_data : ChildData = Default::default();
    expected_data.count_tuple = (0, 1, 2);
    expected_data.proc_name = "proc_1".to_string();
    rcv_and_verify_msg(serv_1, &expected_data);
    expected_data.proc_name = "proc_2".to_string();
    rcv_and_verify_msg(serv_2, &expected_data);

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
