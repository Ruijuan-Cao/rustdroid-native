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


fn find_env_var_path(varname: &'static str) -> Option<PathBuf> {
    match var(varname) {
        Err(_) => None,
        Ok(s) => Some(PathBuf::from(s)).and_then(
            |p| if p.exists() { Some(p) } else { None }),
    }
}

fn find_env_var_ndk_build_path(varname: &'static str) -> Option<PathBuf> {
    find_env_var_path(varname).and_then(
        |p| if p.join("ndk-build").exists() { Some(p) } else { None })
}

fn find_env_var_ndk_bundle_build_path(varname: &'static str) -> Option<PathBuf> {
    // TODO: @@@ DRY THIS WITH ABOVE
    find_env_var_path(varname).and_then(
        |p| if p.join("ndk-bundle").join("ndk-build").exists() { Some(p) } else { None })
}

fn find_ndk_env_var_path() -> Option<PathBuf> {
    // TODO: @@@ REFACTOR INTO ITERATION OF COLLECTION
    find_env_var_ndk_build_path("ANDROID_NDK_HOME")
        .or_else(|| find_env_var_ndk_build_path("ANDROID_NDK_ROOT")
        .or_else(|| find_env_var_ndk_build_path("NDK_HOME")
        .or_else(|| find_env_var_ndk_build_path("NDK_ROOT")     // NVIDIA CodeWorks
        .or_else(|| find_env_var_ndk_build_path("NDKROOT")))))  // NVIDIA CodeWorks
}

fn find_sdk_env_var_path() -> Option<PathBuf> {
    // TODO: @@@ REFACTOR INTO ITERATION OF COLLECTION
    find_env_var_ndk_bundle_build_path("ANDROID_SDK_HOME")
        .or_else(|| find_env_var_ndk_bundle_build_path("ANDROID_SDK_ROOT"))
        .or_else(|| find_env_var_ndk_bundle_build_path("ANDROID_HOME"))
}

fn find_env_ndk_path() -> Option<PathBuf> {
    find_ndk_env_var_path().or_else(|| find_sdk_env_var_path())
}

fn find_ndk_bundle_build_path(pathname: &'static str) -> Option<PathBuf> {
    // TODO: @@@ DRY THIS WITH ABOVE
    let path = PathBuf::from(pathname);
    if path.join("ndk-bundle").join("ndk-build").exists() { Some(path) } else { None }
}

fn find_known_ndk_path() -> Option<PathBuf> {
    find_ndk_bundle_build_path("~/.android/sdk")
        .or_else(|| find_ndk_bundle_build_path("~/Library/Android/sdk")
        .or_else(|| find_ndk_bundle_build_path("~/NVPACK")))
    // TODO: ### NEED TO LOOK FOR ./android-ndk-r???
}

fn find_ndk_build_path() -> Option<PathBuf> {
    find_env_ndk_path().or_else(|| find_known_ndk_path())
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
