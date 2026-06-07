mod friendly_id;

use nix::{
    libc,
    mount::{MntFlags, MsFlags, mount, umount2},
    sched::{CloneFlags, clone},
    sys::{
        stat::{Mode, SFlag, makedev, mknod},
        wait::{WaitStatus, waitpid},
    },
    unistd::{execvp, pivot_root, sethostname},
};

use libc::dev_t;

use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fs::{create_dir_all, remove_dir},
    io,
    os::unix::fs::symlink,
    process::ExitCode,
};

fn extract_prog_args() -> Vec<String> {
    env::args().skip(1).collect()
}

// Hardcoded for now, this should be parameterized later
const ROOT_FS_PATH: &str = "/home/suraj/rootfs/alpine";

fn mount_dev_dirs() -> Result<(), io::Error> {
    let perms_rw_all = Mode::from_bits_truncate(0o666);
    let mut dev_dirs: HashMap<&str, dev_t> = HashMap::new();
    dev_dirs.insert("/dev/null", makedev(1, 3));
    dev_dirs.insert("/dev/zero", makedev(1, 5));
    dev_dirs.insert("/dev/random", makedev(1, 8));
    dev_dirs.insert("/dev/urandom", makedev(1, 9));
    dev_dirs.insert("/dev/tty", makedev(5, 0));

    mount(
        Some("tmpfs"),
        "/dev",
        Some("tmpfs"),
        MsFlags::MS_NOSUID,
        None::<&str>,
    )
    .expect("msg");

    for (path, dev_id) in dev_dirs {
        mknod(path, SFlag::S_IFCHR, perms_rw_all, dev_id)?;
    }

    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
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

        umount2("/old_root", MntFlags::MNT_DETACH).expect("failed unmounting old root.");
        remove_dir("/old_root").expect("failed deleting old root");

        // Mount a new `proc` FS in `/proc`
        let _ = mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::empty(),
            None::<&str>,
        )
        .expect("Failed to mount a new proc");

        let _ = mount(
            Some("sysfs"),
            "/sys",
            Some("sysfs"),
            MsFlags::MS_RDONLY | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
            None::<&str>,
        )
        .expect("Failed to mount a new sysfs");

        let _ = mount(
            Some("tmpfs"),
            "/tmp",
            Some("tmpfs"),
            MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
            None::<&str>,
        )
        .expect("Failed to mount a new tmpfs");

        mount_dev_dirs().expect("failed mounting dev dirs");

        // add better error handling here. if anything fails, write to the pipe. Add to this later.let _ = execvp(&prog_args[0], &prog_args);
        let error = execvp(&prog_args[0], &prog_args);
        eprintln!("EXECVP FAILED: {:?}", error);

        0
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
