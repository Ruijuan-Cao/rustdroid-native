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
            Ok(s) => PathBuf::from(&s).parent()
                .and_then(|p| { Some(p.to_path_buf()) })
        },
    }
}

fn find_environment_ndk_path() -> Option<PathBuf> {
    // TODO: ### IMPLEMENT
    None
}

fn find_known_ndk_path() -> Option<PathBuf> {
    // TODO: ### IMPLEMENT
    Some(PathBuf::from("~/Library/Android/sdk/ndk-bundle"))
        // ^ TODO: ### FOR TESTING
}

fn find_ndk_path() -> Option<PathBuf> {
    find_which_ndk_build_path()
        .or_else(|| find_environment_ndk_path())
        .or_else(|| find_known_ndk_path())
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
