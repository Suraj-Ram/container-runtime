mod friendly_id;

// use std::process::Command;
use nix::{
    libc,
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork, write},
};

use std::ffi::{CStr, CString};

fn main() {
    println!("Hello, world!");
    let c_name = friendly_id::generate();
    println!("Starting container {c_name}");

    // match unsafe { fork() } {
    //     Ok(ForkResult::Parent { child, .. }) => {
    //         println!(
    //             "Continuing execution in parent process, new child has pid: {}",
    //             child
    //         );
    //         waitpid(child, None).unwrap();
    //     }
    //     Ok(ForkResult::Child) => {
    //         // Unsafe to use `println!` (or `unwrap`) here. See Safety.
    //         write(std::io::stdout(), "I'm a new child process\n".as_bytes()).ok();
    //         unsafe { libc::_exit(1) };
    //     }
    //     Err(_) => println!("Fork failed"),
    // }
    //
    //
    // let program: &str = "/bin/ls";
    // let prog_c_string = CString::new(program).unwrap();
    // let prog_c_str = prog_c_string.as_c_str();

    // let _ = execvp(&c"/bin/ls", &[c"-a"]);
    let _ = execvp(&c"/bin/pwd", &[c"-a"]);

    println!("this should not be printed");

    // let _ = execvp(&c"/bin/ls", &[c"-a"]);
}
