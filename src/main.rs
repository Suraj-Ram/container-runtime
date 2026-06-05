mod friendly_id;

// use std::process::Command;
use nix::{
    libc,
    sched::{CloneFlags, clone},
    sys::wait::{WaitStatus, waitpid},
    unistd::{ForkResult, execvp, fork, write},
};

use std::{
    env,
    ffi::{CStr, CString},
    io::Write,
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

    let mut clone_stack = [0u8; 64 * 1024];
    let clone_flags = CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS;

    let clone_child_callback = Box::new(|| {
        println!("Hello from the isolated child process!");
        let _ = execvp(&prog_args[0], &prog_args);
        0 // Exit code 0
    });

    println!("Parent: cloning process...");
    let clone_result = unsafe {
        clone(
            clone_child_callback,
            &mut clone_stack,
            clone_flags,
            Some(libc::SIGCHLD),
        )
    };

    dbg!(clone_result);

    match clone_result {
        Ok(child_pid) => {
            println!("Parent: Successfully spawned child with PID: {}", child_pid);

            // 5. Wait for the child process to finish executing.
            match waitpid(child_pid, None) {
                Ok(WaitStatus::Exited(pid, status)) => {
                    println!("Parent: Child {} exited with status code {}.", pid, status);
                }
                _ => println!("Parent: Something else happened while waiting."),
            }
        }
        Err(err) => {
            eprintln!("Failed to clone process: {}. (Did you forget sudo?)", err);
        }
    }
}
