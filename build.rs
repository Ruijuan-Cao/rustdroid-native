use std::path::PathBuf;
use std::process::Command;

fn find_which_ndk_build_path() -> Option<PathBuf> {
    let mut cmd = Command::new("sh"); // mut due to API limitation
    cmd.arg("-c").arg("which ndk-build");
    match cmd.output() {
        Err(e) => {
            println!(
                "cargo:warning=Error executing process command <{:?}>: {}",
                cmd, e);
            None
        },
        Ok(o) => match String::from_utf8(o.stdout) {
            Err(e) => {
                println!(
                    "cargo:warning=Error parsing path string: {}",
                    e);
                None
            },
            Ok(s) => match PathBuf::from(&s).parent() {
                None => None,
                Some(p) => Some(p.to_path_buf()),
            },
        },
    }
}

fn find_ndk_path() -> Option<PathBuf> {
    // 1. try which command in case environment path is already set
    find_which_ndk_build_path()
    // 2. look for known NDK environment vars
    // TODO: ### IMPLEMENT
    // 2. look for known NDK locations
    // TODO: ### IMPLEMENT
}

fn establish_ndk() {
    match find_ndk_path() {
        None => println!(
            "cargo:warning=NDK path not found"),
        Some(path) => println!(
            "cargo:warning=NDK path found at {}",
            path.to_string_lossy()),
    };
}

fn establish_ndk_toolchain() {
    establish_ndk();
}

fn main() {
    establish_ndk_toolchain();
}
