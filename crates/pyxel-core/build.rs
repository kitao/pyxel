#[cfg(not(test))]
use std::env::var;
#[cfg(not(test))]
use std::fs::File;
use std::io::Read;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use std::process::Command;
#[cfg(not(test))]
use std::str;

use sha2::{Digest, Sha256};
#[cfg(not(test))]
use tar::Archive;

#[cfg(not(test))]
const SDL2_VERSION: &str = "2.32.10"; // Emscripten 5.0.3 uses SDL 2.32.10
#[cfg(not(test))]
const SDL2_SHA256: &str = "5f5993c530f084535c65a6879e9b26ad441169b3e25d789d83287040a9ca5165";

#[cfg(not(test))]
struct Sdl2BindingsBuilder {
    target: String,
    target_os: String,
    sdl2_dir: String,
    out_dir: String,
}

#[cfg(not(test))]
impl Sdl2BindingsBuilder {
    fn new() -> Self {
        let target = var("TARGET").unwrap();
        let target_os = target
            .splitn(3, '-')
            .nth(2)
            .expect("Failed to parse TARGET triple")
            .to_string();
        let out_dir = var("OUT_DIR").unwrap();
        let sdl2_dir = format!("{out_dir}/SDL2-{SDL2_VERSION}");

        Self {
            target,
            target_os,
            sdl2_dir,
            out_dir,
        }
    }

    fn build(&self) {
        if is_sdl2_static() {
            self.download_sdl2();
            self.build_sdl2();
        }

        self.link_sdl2();
        self.generate_bindings();
    }

    // Pipeline steps

    fn download_sdl2(&self) {
        if Path::new(&self.sdl2_dir).exists() {
            return;
        }

        let sdl2_archive_url = format!("https://www.libsdl.org/release/SDL2-{SDL2_VERSION}.tar.gz");
        let sdl2_archive_path = format!("{}/SDL2-{}.tar.gz", self.out_dir, SDL2_VERSION);

        let status = Command::new("curl")
            .arg("-fLo")
            .arg(&sdl2_archive_path)
            .arg(&sdl2_archive_url)
            .status()
            .expect("Failed to execute curl command");
        assert!(status.success(), "Failed to download SDL2 source code");

        // Verify SDL2 before extracting any downloaded content.
        let sdl2_archive = File::open(&sdl2_archive_path).expect("Failed to open SDL2 archive");
        verify_sha256(sdl2_archive, SDL2_SHA256)
            .unwrap_or_else(|err| panic!("Failed to verify SDL2 archive: {err}"));

        let tar_gz = File::open(&sdl2_archive_path).unwrap();
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        if let Err(err) = archive.unpack(&self.out_dir) {
            if Path::new(&self.sdl2_dir).exists() {
                std::fs::remove_dir_all(&self.sdl2_dir).expect("Failed to remove SDL2 directory");
            }
            panic!("Failed to extract SDL2 source code: {err}");
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

        let cmake_install_dir = cfg.build();
        println!(
            "cargo::rustc-link-search={}",
            cmake_install_dir.join("lib64").display()
        );
        println!(
            "cargo::rustc-link-search={}",
            cmake_install_dir.join("lib").display()
        );
    }

    fn link_sdl2(&self) {
        // Static SDL2 needs platform dependencies supplied transitively by dynamic SDL2.
        if is_sdl2_static() {
            println!("cargo::rustc-link-lib=static=SDL2main");
            if self.target_os.contains("windows") {
                println!("cargo::rustc-link-lib=static=SDL2-static");
            } else {
                println!("cargo::rustc-link-lib=static=SDL2");
            }

            if self.target_os.contains("windows") {
                println!("cargo::rustc-link-lib=shell32");
                println!("cargo::rustc-link-lib=user32");
                println!("cargo::rustc-link-lib=gdi32");
                println!("cargo::rustc-link-lib=winmm");
                println!("cargo::rustc-link-lib=imm32");
                println!("cargo::rustc-link-lib=ole32");
                println!("cargo::rustc-link-lib=oleaut32");
                println!("cargo::rustc-link-lib=version");
                println!("cargo::rustc-link-lib=uuid");
                println!("cargo::rustc-link-lib=dinput8");
                println!("cargo::rustc-link-lib=dxguid");
                println!("cargo::rustc-link-lib=setupapi");
            } else if self.target_os == "darwin" {
                println!("cargo::rustc-link-lib=framework=Cocoa");
                println!("cargo::rustc-link-lib=framework=IOKit");
                println!("cargo::rustc-link-lib=framework=Carbon");
                println!("cargo::rustc-link-lib=framework=ForceFeedback");
                println!("cargo::rustc-link-lib=framework=GameController");
                println!("cargo::rustc-link-lib=framework=CoreHaptics");
                println!("cargo::rustc-link-lib=framework=CoreVideo");
                println!("cargo::rustc-link-lib=framework=CoreAudio");
                println!("cargo::rustc-link-lib=framework=AudioToolbox");
                println!("cargo::rustc-link-lib=framework=Metal");
                println!("cargo::rustc-link-lib=iconv");
            }
        } else if self.target_os != "emscripten" {
            println!("cargo::rustc-link-lib=SDL2");
        }
    }

    fn generate_bindings(&self) {
        let mut builder = bindgen::Builder::default()
            .header("wrapper.h")
            .allowlist_function("SDL_.*")
            .allowlist_type("SDL_.*")
            .allowlist_var("SDL_.*")
            .allowlist_var("SDLK_.*")
            .allowlist_var("AUDIO_.*")
            .blocklist_type("_IMAGE_TLS_DIRECTORY64")
            .use_core()
            .generate_comments(false)
            .prepend_enum_name(false)
            .clang_arg(format!("--target={}", self.target))
            .clang_args(self.bindgen_flags())
            .clang_args(self.include_flags());

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

    // Helpers

    fn bindgen_flags(&self) -> Vec<String> {
        if let Ok(bindgen_flags) = var("BINDGENFLAGS") {
            bindgen_flags
                .split_whitespace()
                .map(str::to_string)
                .collect()
        } else {
            Vec::new()
        }
    }

    fn include_flags(&self) -> Vec<String> {
        let mut include_flags = Vec::new();

        if is_sdl2_static() {
            include_flags.push(format!("-I{}/include", self.sdl2_dir));
        } else if self.target_os == "emscripten" {
            let output = Command::new("emcc")
                .args(["--cflags", "--use-port=sdl2"])
                .output()
                .expect("Failed to execute emcc");
            let cflags = str::from_utf8(&output.stdout).unwrap();
            let sdl2_include_flag = cflags
                .split_whitespace()
                .skip_while(|&flag| !flag.starts_with("-isystem"))
                .nth(1)
                .unwrap();

            include_flags.push(format!("-I{sdl2_include_flag}/.."));
            include_flags.push(format!("-I{sdl2_include_flag}"));
        } else {
            for path in [
                "/usr/local/include/SDL2",
                "/usr/include/SDL2",
                "/usr/local/include",
                "/usr/include",
            ] {
                include_flags.push(format!("-I{path}"));
            }
        }

        include_flags
    }
}

pub(crate) fn verify_sha256<R: Read>(mut reader: R, expected: &str) -> Result<(), String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|err| format!("Failed to read archive: {err}"))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let actual: String = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "SHA-256 mismatch: expected {expected}, actual {actual}"
        ))
    }
}

#[cfg(not(test))]
fn has_sdl2_feature() -> bool {
    var("CARGO_FEATURE_SDL2_DYNAMIC").is_ok() || var("CARGO_FEATURE_SDL2_STATIC").is_ok()
}

#[cfg(not(test))]
fn is_sdl2_static() -> bool {
    var("CARGO_FEATURE_SDL2_STATIC").is_ok()
}

#[cfg(not(test))]
fn main() {
    println!("cargo::rustc-check-cfg=cfg(pyxel_core)");
    println!("cargo::rustc-cfg=pyxel_core");
    if has_sdl2_feature() {
        Sdl2BindingsBuilder::new().build();
    }
}
