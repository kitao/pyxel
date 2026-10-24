use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        // The macOS clang runtime can be outside the default linker search path.
        if let Some(path) = macos_link_search_path() {
            println!("cargo::rustc-link-lib=clang_rt.osx");
            println!("cargo::rustc-link-search={path}");
        }
    }
}

fn macos_link_search_path() -> Option<String> {
    let output = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "Failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            return Some(format!("{path}/lib/darwin"));
        }
    }

    println!("Failed to determine link search path, continuing without it");
    None
}
