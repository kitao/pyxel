use array_macro::array;

use crate::image::{Image, SharedImage};
use crate::settings::CAPTURE_FRAME_COUNT;
use crate::Pyxel;

pub struct Resource {
    captured_frames: [(SharedImage, u32); CAPTURE_FRAME_COUNT as usize],
    start_frame_index: u32,
    cur_frame_index: u32,
    frame_count: u32,
}

impl Resource {
    pub fn new(width: u32, height: u32) -> Resource {
        let captured_frames =
            array![_ => (Image::new(width, height), 0); CAPTURE_FRAME_COUNT as usize];

        Resource {
            captured_frames: captured_frames,
            start_frame_index: 0,
            cur_frame_index: 0,
            frame_count: 0,
        }
    }

    /*
    void ClearImage(int32_t image_index);
    void ClearTilemap(int32_t tilemap_index);
    void ClearSound(int32_t sound_index);
    void ClearMusic(int32_t music_index);

    std::string DumpImage(int32_t image_index) const;
    std::string DumpTilemap(int32_t tilemap_index) const;
    std::string DumpSound(int32_t sound_index) const;
    std::string DumpMusic(int32_t music_index) const;

    void ParseImage(int32_t image_index, const std::string& str);
    void ParseTilemap(int32_t tilemap_index, const std::string& str);
    void ParseSound(int32_t sound_index, const std::string& str);
    void ParseMusic(int32_t music_index, const std::string& str);

    static std::string GetVersionName();
    static std::string GetImageName(int32_t image_index);
    static std::string GetTilemapName(int32_t tilemap_index);
    static std::string GetSoundName(int32_t sound_index);
    static std::string GetMusicName(int32_t music_index);
    */
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let _ = (filename, image, tilemap, sound, music); // dummy
        let _ = self.resource.captured_frames; // dummy

        //
    }

    pub fn save(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let _ = (filename, image, tilemap, sound, music); // dummy

        //
    }

    pub fn save_screen_image(&mut self) {
        /*
        SDL_Surface* surface = SDL_CreateRGBSurfaceWithFormat(
            0, width_ * SCREEN_CAPTURE_SCALE, height_ * SCREEN_CAPTURE_SCALE, 32,
            SDL_PIXELFORMAT_RGB888);

        SDL_LockSurface(surface);

        int32_t** src_data = captured_images_[cur_frame_]->Data();
        int32_t* dst_data = reinterpret_cast<int32_t*>(surface->pixels);

        int32_t scaled_width = width_ * SCREEN_CAPTURE_SCALE;
        int32_t scaled_height = height_ * SCREEN_CAPTURE_SCALE;

        for (int32_t i = 0; i < scaled_height; i++) {
          for (int32_t j = 0; j < scaled_width; j++) {
            int32_t index = scaled_width * i + j;
            int32_t color =
                src_data[i / SCREEN_CAPTURE_SCALE][j / SCREEN_CAPTURE_SCALE];

            dst_data[index] = palette_color_[color];
          }
        }

        SDL_UnlockSurface(surface);
        IMG_SavePNG(surface, (GetBaseName() + ".png").c_str());
        SDL_FreeSurface(surface);
        */
    }

    pub fn reset_screen_video(&mut self) {
        self.resource.start_frame_index = (self.resource.cur_frame_index + 1) % CAPTURE_FRAME_COUNT;
        self.resource.frame_count = 0;
    }

    pub fn save_screen_video(&mut self) {
        /*
        std::string filename = GetBaseName() + ".gif";
        GifWriter* gif_writer =
            new GifWriter(filename, width_, height_, palette_color_);

        for (int32_t i = 0; i < frame_count_; i++) {
          int32_t index = (start_frame_ + i) % SCREEN_CAPTURE_COUNT;

          gif_writer->AddFrame(captured_images_[index],
                               captured_frames_[index] * 100.0f / fps_ + 0.5f);
        }

        gif_writer->EndFrame();
        delete gif_writer;

        // try to optimize the generated GIF file with Gifsicle
        int32_t res = system(("gifsicle -b -O3 -Okeep-empty " + filename).c_str());

        ResetScreenCapture();
        */
    }

    pub(crate) fn capture_screen_video(&mut self) {
        if self.resource.frame_count < 1 {
            return;
        }

        /*
        cur_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
        frame_count_++;
        captured_images_[cur_frame_]->CopyImage(0, 0, screen_image, 0, 0, width_,
                                                height_);
        captured_frames_[cur_frame_] = update_frame_count;

        if (frame_count_ > SCREEN_CAPTURE_COUNT) {
          start_frame_ = (start_frame_ + 1) % SCREEN_CAPTURE_COUNT;
          frame_count_ = SCREEN_CAPTURE_COUNT;
        }
        */
    }

    /*
    std::string Recorder::GetBaseName() const {
    #ifdef WIN32
      std::string desktop_path = getenv("USERPROFILE");
      desktop_path += "\\Desktop\\";
    #else
      std::string desktop_path = getenv("HOME");
      desktop_path += "/Desktop/";
    #endif

      char basename[30];
      time_t t = std::time(nullptr);
      std::strftime(basename, sizeof(basename), "pyxel-%y%m%d-%H%M%S",
                    std::localtime(&t));

      return desktop_path + basename;
    }
    */
}
