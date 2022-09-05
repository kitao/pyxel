// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        // On (older) OSX we need to link against the clang runtime,
        // which is hidden in some non-default path.
        //
        // More details at https://github.com/alexcrichton/curl-rust/issues/279.
        if let Some(path) = macos_link_search_path() {
            println!("cargo:rustc-link-lib=clang_rt.osx");
            println!("cargo:rustc-link-search={}", path);
        }
    }
}

fn macos_link_search_path() -> Option<String> {
    let output = cc::Build::new()
        .get_compiler()
        .to_command()
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            if !path.is_empty() {
                return Some(format!("{}/lib/darwin", path));
            }
        }
    }

    println!("failed to determine link search path, continuing without it");
    None
}
