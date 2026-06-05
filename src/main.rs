mod friendly_id;

// use std::process::Command;
use nix::{
    libc,
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork, write},
};

use std::{
    env,
    ffi::{CStr, CString},
};

fn extract_prog_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn main() {
    println!("Hello, world!");
    // let c_name = friendly_id::generate();
    // println!("Starting container {c_name}");

    let prog_args: Vec<CString> = extract_prog_args()
        .into_iter()
        .map(|x| CString::new(x).expect("Received bad string"))
        .collect();

    dbg!(&prog_args);

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            // write(std::io::stdout(), "I'm a new child process\n".as_bytes()).ok();
            println!("in child");
            let _ = execvp(&prog_args[0], &prog_args);
        }
        Err(_) => println!("Fork failed"),
    }
}
