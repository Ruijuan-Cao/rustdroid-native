use std::path::Path;
use std::process::Command;

fn main() {
    // find the NDK
    let whichndk = match Command::new("sh").arg("-c")
            .arg("which ndk-build").output() {
        Ok(o) => String::from_utf8(o.stdout).unwrap_or(String::new()),
        Err(e) => {
            println!("cargo:warning=ERROR executing command: {}", e);
            String::new()
        },
    };
    if ! whichndk.is_empty() {
        let pathndk = Path::new(&whichndk).parent().unwrap_or(Path::new(""));
        println!("cargo:warning=NDK found at path {}",
            pathndk.to_string_lossy());
    } else {
        println!("cargo:warning=NDK not found");
    }
}
