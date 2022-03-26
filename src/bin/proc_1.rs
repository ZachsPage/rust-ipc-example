use ipc_comms::ipc_demo::run_demo_child;

fn main() {
    let ret_code = match run_demo_child("proc_1") {
        Ok(_) => 0,
        Err(err) => { eprintln!("App Error {:?}", err); 1 }
    };
    std::process::exit( ret_code )
}