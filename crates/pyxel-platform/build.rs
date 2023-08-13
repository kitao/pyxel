use std::fs::File;
use std::io::copy;
use std::io::BufWriter;
use std::path::Path;

use cmake::Config;
use tar::Archive;

const SDL2_VERSION: &str = "2.28.1";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let sdl2_dir = format!("{}/SDL2-{}", out_dir, SDL2_VERSION);
    let sdl2_archive_url = format!(
        "https://www.libsdl.org/release/SDL2-{}.tar.gz",
        SDL2_VERSION
    );
    let sdl2_archive_path = format!("{}/SDL2-{}.tar.gz", out_dir, SDL2_VERSION);

    // download and extract SDL2
    if !Path::new(&sdl2_dir).exists() {
        // download SDL2
        let mut resp =
            reqwest::blocking::get(sdl2_archive_url).expect("Failed to download SDL2 source code");
        let file = File::create(&sdl2_archive_path).unwrap();
        let mut writer = BufWriter::new(file);
        copy(&mut resp, &mut writer).expect("Failed to write SDL2 source code to file");

        // extract SDL2
        let tar_gz = File::open(&sdl2_archive_path).unwrap();
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        if let Err(_err) = archive.unpack(&out_dir) {
            if Path::new(&sdl2_dir).exists() {
                std::fs::remove_dir_all(&sdl2_dir).expect("Failed to remove SDL2 directory");
            }
            panic!("Failed to extract SDL2 source code");
        }
    }

    // build SDL2
    let mut cfg = Config::new(&sdl2_dir);
    cfg.profile("release");
    cfg.cflag("-D__FLTUSED__");
    cfg.define("SDL_SHARED", "OFF");
    cfg.define("SDL_STATIC", "ON");
    cfg.define("SDL_MAIN_HANDLED", "ON");
    cfg.build();

    // generate bindings
    bindgen::Builder::default()
        .header(format!("{}/include/SDL.h", sdl2_dir))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("Failed to write bindings");
}
