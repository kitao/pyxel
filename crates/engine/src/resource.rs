use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use crate::capturer::Capturer;
use crate::image::Image;
use crate::music::Music;
use crate::settings::{
    IMAGE_COUNT, MUSIC_COUNT, RESOURCE_ARCHIVE_DIRNAME, SOUND_COUNT, TILEMAP_COUNT,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::utils::{parse_version_string, pyxel_version};
use crate::Pyxel;

pub struct Resource {
    capturer: Capturer,
}

impl Resource {
    pub fn new(width: u32, height: u32, capture_frame_count: u32) -> Resource {
        Resource {
            capturer: Capturer::new(width, height, capture_frame_count),
        }
    }

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        self.capturer.capture_screen(screen, frame_count);
    }
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(File::open(&Path::new(filename)).unwrap()).unwrap();

        {
            let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            if parse_version_string(&contents).unwrap() > pyxel_version() {
                panic!("unsupported resource file version '{}'", contents);
            }
        }

        macro_rules! deserialize {
            ($type: ty, $getter: ident, $count: expr) => {
                for i in 0..$count {
                    if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                        let mut input = String::new();
                        file.read_to_string(&mut input).unwrap();
                        self.$getter(i).lock().deserialize(&input);
                    } else {
                        self.$getter(i).lock().clear();
                    }
                }
            };
        }

        if image {
            deserialize!(Image, image, IMAGE_COUNT);
        }

        if tilemap {
            deserialize!(Tilemap, tilemap, TILEMAP_COUNT);
        }

        if sound {
            deserialize!(Sound, sound, SOUND_COUNT);
        }

        if music {
            deserialize!(Music, music, MUSIC_COUNT);
        }
    }

    pub fn save(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        // TODO
        let _ = (filename, image, tilemap, sound, music); // dummy

        /*
        std::ofstream ofs(std::filesystem::u8path(filename), std::ios::binary);

        if (ofs.fail()) {
          PYXEL_ERROR("cannot save file '" + filename + "'");
        }

        miniz_cpp::zip_file file;

        file.writestr(GetVersionName(), VERSION + '\n');

        for (int32_t i = 0; i < USER_IMAGE_BANK_COUNT; i++) {
          std::string str = DumpImage(i);

          if (str.size() > 0) {
            file.writestr(GetImageName(i), str);
          }
        }

        for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
          std::string str = DumpTilemap(i);

          if (str.size() > 0) {
            file.writestr(GetTilemapName(i), str);
          }
        }

        for (int32_t i = 0; i < USER_SOUND_BANK_COUNT; i++) {
          std::string str = DumpSound(i);

          if (str.size() > 0) {
            file.writestr(GetSoundName(i), str);
          }
        }

        for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
          std::string str = DumpMusic(i);

          if (str.size() > 0) {
            file.writestr(GetMusicName(i), str);
          }
        }

        file.save(ofs);
        ofs.close();
        */
    }

    pub fn screenshot(&mut self) {
        self.resource.capturer.screenshot(&self.colors);
    }

    pub fn reset_capture(&mut self) {
        self.resource.capturer.reset_capture();
    }

    pub fn screencast(&mut self) {
        self.resource.capturer.screencast(&self.colors);
    }
}
