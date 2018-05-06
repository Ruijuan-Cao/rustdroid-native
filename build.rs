use std::{env, path::PathBuf, process};

fn command_which_ndk_build_path() -> Option<PathBuf> {
    let mut cmd = process::Command::new("sh"); // mut due to API limitation
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
                    "cargo:warning=Error parsing command output as UTF-8: {}",
                    e);
                None
            },
            Ok(s) => PathBuf::from(&s).parent()
                .and_then(|p| { Some(p.to_path_buf()) })
        },
    }
}

fn path_from_string(pathname: &str) -> Option<PathBuf> {
    // TODO: @@@ FUTURE RUST FEATURE
    //Some(PathBuf::from(pathname)).filter(|p| p.exists())
    let path = PathBuf::from(&pathname);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn path_from_env_var(varname: &'static str) -> Option<PathBuf> {
    match env::var(varname) {
        Ok(s) => path_from_string(&s),
        Err(_) => None,
    }
}

fn path_with_ndk_build(path: &PathBuf) -> Option<PathBuf> {
    // TODO: @@@ FUTURE RUST FEATURE
    //path.filter(|p| p.join("ndk-build").exists())
    if path.join("ndk-build").exists() {
        Some(path.clone())
    } else {
        None
    }
}

fn path_with_ndk_bundle_ndk_build(path: &PathBuf) -> Option<PathBuf> {
    path_with_ndk_build(&path.join("ndk-bundle"))
}

fn path_with_ndk_build_from_env_var(varname: &'static str) -> Option<PathBuf> {
    path_from_env_var(&varname).and_then(|p| path_with_ndk_build(&p))
}

fn path_with_ndk_bundle_ndk_build_from_env_var(varname: &'static str) -> Option<PathBuf> {
    path_from_env_var(&varname).and_then(|p| path_with_ndk_bundle_ndk_build(&p))
}

fn find_ndk_path_from_ndk_env_vars() -> Option<PathBuf> {
    // TODO: @@@ REFACTOR INTO ITERATION OF COLLECTION
    path_with_ndk_build_from_env_var("ANDROID_NDK_HOME")
        .or_else(|| path_with_ndk_build_from_env_var("ANDROID_NDK_ROOT")
        .or_else(|| path_with_ndk_build_from_env_var("NDK_HOME")
        .or_else(|| path_with_ndk_build_from_env_var("NDK_ROOT")     // NVIDIA CodeWorks
        .or_else(|| path_with_ndk_build_from_env_var("NDKROOT")))))  // NVIDIA CodeWorks
}

fn find_ndk_path_from_sdk_env_vars() -> Option<PathBuf> {
    // TODO: @@@ REFACTOR INTO ITERATION OF COLLECTION
    path_with_ndk_bundle_ndk_build_from_env_var("ANDROID_SDK_HOME")
        .or_else(|| path_with_ndk_bundle_ndk_build_from_env_var("ANDROID_SDK_ROOT"))
        .or_else(|| path_with_ndk_bundle_ndk_build_from_env_var("ANDROID_HOME"))
}

fn find_ndk_path_from_env_vars() -> Option<PathBuf> {
    find_ndk_path_from_ndk_env_vars().or_else(
        || find_ndk_path_from_sdk_env_vars())
}

fn find_ndk_version_build_path(path: &PathBuf) -> Option<PathBuf> {
    //println!("cargo:warning=find_ndk_version_build_path() pathname: {:?}", pathname);
    if let Ok(iter) = path.read_dir() {
        for entry in iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                //println!("cargo:warning=searching path: {:?}", path);
                if path.join("ndk-build").exists() {
                    return Some(path)
                }
            }
        }
    }
    None
}

fn find_ndk_path_from_known_installations() -> Option<PathBuf> {
    env::home_dir().and_then(|home|
        path_with_ndk_bundle_ndk_build(
            // Android Studio on GNU/Linux
            &home.join(".android").join("sdk")
        ).or_else(|| path_with_ndk_bundle_ndk_build(
            // Android Studio on macOS
            &home.join("Library").join("Android").join("sdk"))
        ).or_else(|| find_ndk_version_build_path(
            // NVIDIA CodeWorks
            &home.join("NVPACK"))
        )
    )
}

fn find_ndk_path() -> Option<PathBuf> {
    command_which_ndk_build_path()
        .or_else(|| find_ndk_path_from_env_vars())
        .or_else(|| find_ndk_path_from_known_installations())
}

fn find_ndk_toolchain_path() -> Option<PathBuf> {
    path_from_env_var("NDK_TOOLCHAIN")
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
    match find_ndk_toolchain_path() {
        None => println!(
            "cargo:warning=NDK_TOOLCHAIN path not found"),
        Some(path) => println!(
            "cargo:warning=NDK_TOOLCHAIN path found at {}",
            path.to_string_lossy()),
    };
}

fn main() {
    establish_ndk();
    establish_ndk_toolchain();
}
