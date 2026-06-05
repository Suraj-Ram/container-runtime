mod friendly_id;

use nix::{
    libc,
    sched::{CloneFlags, clone},
    sys::wait::{WaitStatus, waitpid},
    unistd::{execvp, sethostname},
};

use std::{env, ffi::CString, process::ExitCode};

fn extract_prog_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn main() -> ExitCode {
    println!("Hello, world!");
    let c_name = friendly_id::generate();
    println!("Starting container {c_name}");

    let prog_args: Vec<CString> = extract_prog_args()
        .into_iter()
        .map(|x| CString::new(x).expect("Received bad string"))
        .collect();

    dbg!(&prog_args);

    let mut clone_stack = [0u8; 64 * 1024];
    let clone_flags = CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS;

    let clone_child_callback = Box::new(|| {
        println!("Hello from the isolated child process!");
        sethostname("container-hostname").expect("Failed to set hostname");
        // add error handling here. if anything fails, write to the pipe. Add to this later.
        let _ = execvp(&prog_args[0], &prog_args);
        eprintln!("EXECVP FAILED.");

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

    // dbg!(clone_result);

    match clone_result {
        Ok(child_pid) => {
            println!("Parent: Successfully spawned child with PID: {}", child_pid);

            // 5. Wait for the child process to finish executing.
            match waitpid(child_pid, None) {
                Ok(WaitStatus::Exited(pid, status)) => {
                    println!("Parent: Child {} exited with status code {}.", pid, status);
                    match status {
                        0 => ExitCode::SUCCESS,
                        _ => ExitCode::FAILURE,
                    }
                }
                _ => {
                    println!("Parent: Something else happened while waiting.");
                    ExitCode::FAILURE
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to clone process: {}. (Did you forget sudo?)", err);
            ExitCode::FAILURE
        }
    }
}
