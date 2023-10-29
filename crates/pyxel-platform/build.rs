use std::env::var;
use std::fs::{write, File};
use std::io::{copy, BufWriter};
use std::path::Path;

use tar::Archive;

const SDL2_VERSION: &str = "2.28.2";

fn main() {
    let target = var("TARGET").unwrap();
    let target_os = target.splitn(3, '-').nth(2).unwrap();
    let out_dir = &var("OUT_DIR").unwrap();
    let sdl2_dir = &format!("{}/SDL2-{}", out_dir, SDL2_VERSION);

    download_sdl2(sdl2_dir, out_dir);
    build_sdl2(sdl2_dir, target_os);
    generate_bindings(sdl2_dir, out_dir);
}

fn download_sdl2(sdl2_dir: &str, out_dir: &str) {
    if Path::new(&sdl2_dir).exists() {
        return;
    }
    let sdl2_archive_url = format!(
        "https://www.libsdl.org/release/SDL2-{}.tar.gz",
        SDL2_VERSION
    );
    let sdl2_archive_path = format!("{}/SDL2-{}.tar.gz", out_dir, SDL2_VERSION);

    // Download SDL2
    let mut resp =
        reqwest::blocking::get(sdl2_archive_url).expect("Failed to download SDL2 source code");
    let file = File::create(&sdl2_archive_path).unwrap();
    let mut writer = BufWriter::new(file);
    copy(&mut resp, &mut writer).expect("Failed to write SDL2 source code to file");

    // Extract SDL2
    let tar_gz = File::open(&sdl2_archive_path).unwrap();
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    if let Err(_err) = archive.unpack(out_dir) {
        if Path::new(&sdl2_dir).exists() {
            std::fs::remove_dir_all(sdl2_dir).expect("Failed to remove SDL2 directory");
        }
        panic!("Failed to extract SDL2 source code");
    }
}

fn build_sdl2(sdl2_dir: &str, target_os: &str) {
    // Compile SDL2
    let mut cfg = cmake::Config::new(sdl2_dir);
    cfg.profile("release");
    if target_os == "linux" {
        /*
        use version_compare::{compare_to, Cmp};
        if let Ok(version) = std::process::Command::new("cc")
            .arg("-dumpversion")
            .output()
        {
            let affected =
                compare_to(std::str::from_utf8(&version.stdout).unwrap(), "10", Cmp::Ge)
                    .unwrap_or(true);
            if affected {
                cfg.cflag("-fcommon");
            }
        }
        */
    } else {
        cfg.cflag("-D__FLTUSED__");
    }
    if target_os == "windows-gnu" {
        cfg.define("VIDEO_OPENGLES", "OFF");
    }
    cfg.define("SDL_SHARED", "OFF");
    cfg.define("SDL_STATIC", "ON");
    cfg.define("SDL_MAIN_HANDLED", "ON");
    let sdl2_compiled_path = cfg.build();

    // Link SDL2
    println!(
        "cargo:rustc-link-search={}",
        sdl2_compiled_path.join("lib64").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        sdl2_compiled_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=SDL2main");
    if target_os.contains("windows") {
        println!("cargo:rustc-link-lib=static=SDL2-static");
    } else {
        println!("cargo:rustc-link-lib=static=SDL2");
    }
    if target_os.contains("windows") {
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=imm32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=version");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=dinput8");
        println!("cargo:rustc-link-lib=dxguid");
        println!("cargo:rustc-link-lib=setupapi");
    } else if target_os == "darwin" {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Carbon");
        println!("cargo:rustc-link-lib=framework=ForceFeedback");
        println!("cargo:rustc-link-lib=framework=GameController");
        println!("cargo:rustc-link-lib=framework=CoreHaptics");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=iconv");
    }
}

fn generate_bindings(sdl2_dir: &str, out_dir: &str) {
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/SDL.h", sdl2_dir))
        .generate()
        .expect("Failed to generate bindings")
        .to_string()
        .replace("SDL_EventType_", "")
        .replace("SDL_GLattr_", "")
        .replace("SDL_GLprofile_", "")
        .replace("SDL_KeyCode_", "")
        .replace("SDL_PixelFormatEnum_", "")
        .replace("SDL_WindowEventID_", "")
        .replace("SDL_WindowFlags_", "");
    write(Path::new(&out_dir).join("bindings.rs"), bindings).expect("Failed to write bindings");
}
