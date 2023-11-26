use std::env::var;
use std::fs::File;
use std::io::{copy, BufWriter};
use std::path::Path;
use std::process::Command;
use std::str;

use tar::Archive;

const SDL2_VERSION: &str = "2.24.2";

struct SDL2BindingsBuilder {
    target: String,
    target_os: String,
    sdl2_dir: String,
    out_dir: String,
}

impl SDL2BindingsBuilder {
    fn new() -> Self {
        let target = var("TARGET").unwrap();
        let target_os = target.splitn(3, '-').nth(2).unwrap().to_string();
        let out_dir = var("OUT_DIR").unwrap();
        let sdl2_dir = format!("{}/SDL2-{}", out_dir, SDL2_VERSION);
        Self {
            target,
            target_os,
            sdl2_dir,
            out_dir,
        }
    }

    fn should_bundle_sdl2(&self) -> bool {
        self.target_os.contains("windows") || self.target_os == "darwin"
    }

    fn download_sdl2(&self) {
        if Path::new(&self.sdl2_dir).exists() {
            return;
        }
        let sdl2_archive_url = format!(
            "https://www.libsdl.org/release/SDL2-{}.tar.gz",
            SDL2_VERSION
        );
        let sdl2_archive_path = format!("{}/SDL2-{}.tar.gz", self.out_dir, SDL2_VERSION);

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
        if let Err(_err) = archive.unpack(&self.out_dir) {
            if Path::new(&self.sdl2_dir).exists() {
                std::fs::remove_dir_all(&self.sdl2_dir).expect("Failed to remove SDL2 directory");
            }
            panic!("Failed to extract SDL2 source code");
        }
    }

    fn build_sdl2(&self) {
        let mut cfg = cmake::Config::new(&self.sdl2_dir);
        cfg.profile("release")
            .cflag("-D__FLTUSED__")
            .define("SDL_SHARED", "OFF")
            .define("SDL_STATIC", "ON")
            .define("SDL_MAIN_HANDLED", "ON");
        if self.target_os == "windows-gnu" {
            cfg.define("VIDEO_OPENGLES", "OFF");
        }
        let cmake_out_dir = cfg.build();
        println!(
            "cargo:rustc-link-search={}",
            cmake_out_dir.join("lib64").display()
        );
        println!(
            "cargo:rustc-link-search={}",
            cmake_out_dir.join("lib").display()
        );
    }

    fn link_sdl2(&self) {
        if self.should_bundle_sdl2() {
            println!("cargo:rustc-link-lib=static=SDL2main");
            if self.target_os.contains("windows") {
                println!("cargo:rustc-link-lib=static=SDL2-static");
            } else {
                println!("cargo:rustc-link-lib=static=SDL2");
            }
            if self.target_os.contains("windows") {
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
            } else if self.target_os == "darwin" {
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
        } else if self.target_os != "emscripten" {
            println!("cargo:rustc-flags=-l SDL2");
        }
    }

    fn get_include_paths(&self) -> Vec<String> {
        let mut include_paths = Vec::new();
        if self.should_bundle_sdl2() {
            include_paths.push(format!("-I{}/include", self.sdl2_dir));
        } else if self.target_os == "emscripten" {
            let output = Command::new("emcc")
                .args(["--cflags", "-s", "USE_SDL=2"])
                .output()
                .unwrap();
            let cflags_str = str::from_utf8(&output.stdout).unwrap();
            let sdl2_include_paths = cflags_str
                .split_whitespace()
                .filter(|flag| flag.starts_with("-I"))
                .collect::<Vec<&str>>();
            let sdl2_include_path = sdl2_include_paths
                .iter()
                .find(|&&path| path.contains("SDL2"))
                .map(|&path| path[2..].to_string())
                .unwrap();
            include_paths.push(format!("-I{}", sdl2_include_path));
            include_paths.push(format!("-I{}/..", sdl2_include_path));
        } else {
            include_paths.push(format!("-I{}", var("SDL2_INCLUDE_PATH").unwrap()));
        }
        include_paths
    }

    fn generate_bindings(&self) {
        let mut builder = bindgen::Builder::default()
            .header("wrapper.h")
            .allowlist_function("SDL_.*")
            .allowlist_type("SDL_.*")
            .allowlist_var("SDL_.*")
            .allowlist_var("AUDIO_.*")
            .blocklist_type("_IMAGE_TLS_DIRECTORY64")
            .use_core()
            .prepend_enum_name(false)
            .clang_arg(format!("--target={}", self.target.clone()))
            .clang_args(self.get_include_paths());
        if self.target_os == "windows-msvc" {
            builder = builder
                .clang_arg("-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared")
                .clang_arg("-IC:/Program Files/LLVM/lib/clang/5.0.0/include")
                .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt")
                .clang_arg("-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include")
                .clang_arg("-IC:/Program Files (x86)/Windows Kits/8.1/Include/um");
        }
        if self.target_os == "linux-gnu" {
            builder = builder
                .clang_arg("-DSDL_VIDEO_DRIVER_X11")
                .clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
        builder
            .generate()
            .expect("Failed to generate bindings")
            .write_to_file(Path::new(&self.out_dir).join("bindings.rs"))
            .unwrap();
    }

    fn build(&self) {
        if self.should_bundle_sdl2() {
            self.download_sdl2();
            self.build_sdl2();
        }
        self.link_sdl2();
        self.generate_bindings()
    }
}

fn main() {
    SDL2BindingsBuilder::new().build();
}
