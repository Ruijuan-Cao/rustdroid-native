use std::env::var;
use std::path::PathBuf;
use std::process::Command;

fn command_which_ndk_build_path() -> Option<PathBuf> {
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

fn find_ndk_env_var_string() -> Option<String> {
    var("ANDROID_NDK_HOME")
        .or_else(|_| var("NDK_HOME")
        .or_else(|_| var("NDK_ROOT")
        .or_else(|_| var("NDKROOT"))))
        .ok()
}

fn find_sdk_env_var_string() -> Option<String> {
    var("ANDROID_SDK_HOME")
        .or_else(|_| var("ANDROID_HOME"))
        .ok()
}

fn find_env_ndk_path() -> Option<PathBuf> {
    match find_ndk_env_var_string().or_else(
        || find_sdk_env_var_string().map(|s| s + "ndk-bundle")) {
        Some(s) => Some(PathBuf::from(s)),
        None => None,
    }
}

fn find_known_ndk_path() -> Option<PathBuf> {
    // TODO: @@@ MAKE THIS RUSTY
    let pathandroidstudio = PathBuf::from("~/Library/Android/sdk/ndk-bundle");
    if pathandroidstudio.exists() {
        Some(pathandroidstudio)
    } else {
        let pathnvidiacodeworks = PathBuf::from("~/NVPACK");
        if pathnvidiacodeworks.exists() {
            Some(pathnvidiacodeworks)
        } else {
            None
        }
    }
}

fn find_ndk_build_path() -> Option<PathBuf> {
    find_env_ndk_path().or_else(
        || find_known_ndk_path().and_then(
            |p| if p.join("bin").join("ndk-build").exists() {
                Some(p)
            } else {
                None
            }
        )
    )
}

fn find_ndk_path() -> Option<PathBuf> {
    command_which_ndk_build_path()
        .or_else(|| find_ndk_build_path())
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
