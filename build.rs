use std::path::PathBuf;
use std::process::Command;

fn find_which_ndk_build_path() -> PathBuf {
    let mut ndkpath = PathBuf::new();
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg("which ndk-build");
    match cmd.output() {
        Err(e) => {
            println!("cargo:warning=Error executing process command <{:?}>: {}",
                cmd, e);
        },
        Ok(o) => {
            match String::from_utf8(o.stdout) {
                Err(e) => {
                    println!("cargo:warning=Error parsing path string: {}",
                        e);
                },
                Ok(s) => {
                    ndkpath = PathBuf::from(&s);
                    ndkpath.pop();
                },
            }
        },
    };
    ndkpath
}

fn find_ndk_path() -> PathBuf {
    find_which_ndk_build_path()
}

fn show_ndk_path(ndkpath: PathBuf) {
    let ndkpathstr = ndkpath.to_string_lossy();
    println!("cargo:warning=NDK path {}{}",
        if ndkpathstr.is_empty() { "not found" } else { "found at " },
        ndkpathstr);
}

fn establish_ndk() {
    let ndkpath = find_ndk_path();
    show_ndk_path(ndkpath);
}

fn establish_ndk_toolchain() {
    establish_ndk();
}

fn main() {
    establish_ndk_toolchain();
}
