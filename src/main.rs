mod friendly_id;

use nix::{
    libc,
    mount::{MsFlags, mount},
    sched::{CloneFlags, clone},
    sys::wait::{WaitStatus, waitpid},
    unistd::{execvp, pivot_root, sethostname},
};

use std::{env, ffi::CString, fs::create_dir_all, process::ExitCode};

fn extract_prog_args() -> Vec<String> {
    env::args().skip(1).collect()
}

// Hardcoded for now, this should be parameterized later
const ROOT_FS_PATH: &str = "/home/suraj/rootfs/alpine";

fn main() -> ExitCode {
    println!("Hello, world!");
    let c_name = friendly_id::generate();
    println!("Starting container {c_name}");

    let prog_args: Vec<CString> = extract_prog_args()
        .into_iter()
        .map(|x| CString::new(x).expect("Received bad string"))
        .collect();

    dbg!(&prog_args);

    // Create old roout mount on the host
    let old_root_path = format!("{}/old_root", ROOT_FS_PATH);
    create_dir_all(&old_root_path).expect("failed to create old root path");

    let mut clone_stack = [0u8; 64 * 1024];
    let clone_flags = CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS;

    let clone_child_callback = Box::new(|| {
        println!("Hello from the isolated child process!");

        // Set mount progogation to private
        let _ = mount(
            None::<&str>,                          // source: none
            "/",                                   // target: the entire tree
            None::<&str>,                          // fstype: none
            MsFlags::MS_REC | MsFlags::MS_PRIVATE, // recursive, private
            None::<&str>,                          // data: none
        )
        .expect("Failed to set mount propogation");

        // Mount root fs into itself to make it a bind mount, so pivot-root can use it
        let _ = mount(
            Some(ROOT_FS_PATH),
            ROOT_FS_PATH,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )
        .expect("Failed to set bind mount");

        sethostname("container-hostname").expect("Failed to set hostname");
        let _ = pivot_root(ROOT_FS_PATH, old_root_path.as_str()).unwrap();
        std::env::set_current_dir("/").expect("setting current dir failed");

        // Mount a new `proc` FS in `/proc`
        let _ = mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::empty(),
            None::<&str>,
        )
        .expect("Failed to mount a new proc");

        // UNCOMMENT FOR DEBUG
        // println!("Printing entries in container root");
        // let entries = std::fs::read_dir("/").unwrap();
        // for entry in entries {
        //     let p = entry.unwrap().path();
        //     println!("{p:?}");
        // }

        // add better error handling here. if anything fails, write to the pipe. Add to this later.let _ = execvp(&prog_args[0], &prog_args);
        let error = execvp(&prog_args[0], &prog_args);
        eprintln!("EXECVP FAILED: {:?}", error);

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
