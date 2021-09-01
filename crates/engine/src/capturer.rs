use chrono::Local;
use image::imageops::{self, FilterType};
use image::{Rgb, RgbImage};
use std::env;
use std::path::Path;

use crate::canvas::Canvas;
use crate::image::Image;
use crate::settings::CAPTURE_SCALE;
use crate::types::Rgb8;

struct CaptureFrame {
    frame_image: Image,
    frame_count: u32,
}

pub struct Capturer {
    capture_frame_count: u32,
    capture_frames: Vec<CaptureFrame>,
    start_frame_index: u32,
    cur_frame_index: u32,
    cur_frame_count: u32,
}

impl Capturer {
    pub fn new(width: u32, height: u32, capture_frame_count: u32) -> Capturer {
        let capture_frames = (0..capture_frame_count)
            .map(|_| CaptureFrame {
                frame_image: Image::without_arc_mutex(width, height),
                frame_count: 0,
            })
            .collect();

        Capturer {
            capture_frame_count: capture_frame_count,
            capture_frames: capture_frames,
            start_frame_index: 0,
            cur_frame_index: 0,
            cur_frame_count: 0,
        }
    }

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        if self.capture_frame_count == 0 {
            return;
        }

        self.cur_frame_index = (self.cur_frame_index + 1) % self.capture_frame_count;
        self.cur_frame_count += 1;

        self.capture_frames[self.cur_frame_index as usize]
            .frame_image
            .blt(
                0,
                0,
                screen,
                0,
                0,
                screen.width() as i32,
                screen.height() as i32,
                None,
            );
        self.capture_frames[self.cur_frame_index as usize].frame_count = frame_count;

        if self.cur_frame_count > self.capture_frame_count {
            self.start_frame_index = (self.start_frame_index + 1) % self.capture_frame_count;
            self.cur_frame_count = self.capture_frame_count;
        }
    }

    pub fn screenshot(&mut self, colors: &[Rgb8]) {
        let screen = &self.capture_frames[self.cur_frame_index as usize].frame_image;
        let width = screen.width();
        let height = screen.height();
        let mut image = RgbImage::new(width, height);

        for i in 0..height {
            for j in 0..width {
                let rgb = colors[screen._value(j as i32, i as i32) as usize];
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;

                image.put_pixel(j, i, Rgb([r, g, b]));
            }
        }

        let image = imageops::resize(
            &image,
            width * CAPTURE_SCALE,
            height * CAPTURE_SCALE,
            FilterType::Nearest,
        );

        image.save(Capturer::export_path() + ".png").unwrap();
    }

    pub fn reset_capture(&mut self) {
        if self.capture_frame_count == 0 {
            return;
        }

        self.start_frame_index = (self.cur_frame_index + 1) % self.capture_frame_count;
        self.cur_frame_count = 0;
    }

    pub fn screencast(&mut self, colors: &[Rgb8]) {
        if self.capture_frame_count == 0 || self.cur_frame_count == 0 {
            return;
        }

        // TODO
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

    #[cfg(not(target_os = "windows"))]
    fn export_path() -> String {
        Path::new(&env::var("HOME").unwrap())
            .join("Desktop")
            .join(Local::now().format("pyxel-%Y%m%d-%H%M%S").to_string())
            .to_str()
            .unwrap()
            .to_string()
    }

    #[cfg(target_os = "windows")]
    fn export_path() -> String {
        Path::new(&env::var("USERPROFILE").unwrap())
            .join(RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
                .unwrap()
                .get_value("Desktop")
                .unwrap())
    }
}

/*
#include "pyxelcore/recorder.h"

#include "pyxelcore/gif_writer.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Recorder::Recorder(int32_t width,
                   int32_t height,
                   const PaletteColor& palette_color,
                   int32_t fps) {
  width_ = width;
  height_ = height;
  palette_color_ = palette_color;
  fps_ = fps;
  cur_frame_ = -1;
  start_frame_ = 0;
  frame_count_ = 0;

  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    captured_images_[i] = new Image(width, height);
  }
}

Recorder::~Recorder() {
  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    delete captured_images_[i];
  }
}

void Recorder::SaveScreenshot() {
  if (frame_count_ < 1) {
    return;
  }

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
}

void Recorder::ResetScreenCapture() {
  start_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_ = 0;
}

void Recorder::SaveScreenCapture() {
  if (frame_count_ < 1) {
    return;
  }

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
}

void Recorder::Update(const Image* screen_image, int32_t update_frame_count) {
  cur_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_++;
  captured_images_[cur_frame_]->CopyImage(0, 0, screen_image, 0, 0, width_,
                                          height_);
  captured_frames_[cur_frame_] = update_frame_count;

  if (frame_count_ > SCREEN_CAPTURE_COUNT) {
    start_frame_ = (start_frame_ + 1) % SCREEN_CAPTURE_COUNT;
    frame_count_ = SCREEN_CAPTURE_COUNT;
  }
}

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

}  // namespace pyxelcore
*/

/*
#include "pyxelcore/gif_writer.h"

#include "pyxelcore/image.h"

namespace pyxelcore {

const int32_t TRANSPARENT_COLOR = COLOR_COUNT;
const int32_t VALUE_TYPE_COUNT = COLOR_COUNT + 1;
const int32_t MIN_CODE_SIZE = 5;
const int32_t MAX_CODE_SIZE = 12;
const int32_t MAX_CODE_COUNT = 1 << MAX_CODE_SIZE;
const int32_t MAX_BLOCK_SIZE = 255;
const int32_t CLEAR_CODE = 1 << MIN_CODE_SIZE;

template <class T, size_t N1, size_t N2>
void ClearCodeTree(T (&code_tree)[N1][N2]) {
  for (int32_t i = 0; i < N1; i++) {
    for (int32_t j = 0; j < N2; j++) {
      code_tree[i][j] = 0;
    }
  }
}

class ImageDataBlock {
 public:
  ImageDataBlock(std::ofstream* ofs) {
    ofs_ = ofs;
    bit_index_ = 0;
    bit_data_ = 0;
    block_size_ = 0;
  }

  void AddCode(int32_t code, int32_t size) {
    for (int32_t i = 0; i < size; i++) {
      WriteBit(code);
      code >>= 1;
    }
  }

  void EndCode() {
    while (bit_index_ > 0) {
      WriteBit(0);
    }

    if (block_size_ > 0) {
      WriteBlock();
    }
  }

 private:
  std::ofstream* ofs_;
  int32_t bit_index_;
  int32_t bit_data_;
  int32_t block_size_;
  uint8_t block_data_[MAX_BLOCK_SIZE];

  void WriteBit(int32_t bit) {
    bit_data_ |= (bit & 1) << bit_index_;
    bit_index_++;

    if (bit_index_ == 8) {
      if (block_size_ >= MAX_BLOCK_SIZE) {
        PYXEL_ERROR("failed to generate GIF");
      }

      block_data_[block_size_] = bit_data_;
      block_size_++;

      bit_index_ = 0;
      bit_data_ = 0;

      if (block_size_ == MAX_BLOCK_SIZE) {
        WriteBlock();
      }
    }
  }

  void WriteBlock() {
    ofs_->put(block_size_);
    ofs_->write(reinterpret_cast<char*>(block_data_), block_size_);

    bit_index_ = 0;
    bit_data_ = 0;
    block_size_ = 0;
  }
};

GifWriter::GifWriter(const std::string& filename,
                     int32_t width,
                     int32_t height,
                     const PaletteColor& palette_color) {
  ofs_ =
      std::ofstream(std::filesystem::u8path(filename), std::ios_base::binary);
  width_ = width;
  height_ = height;
  last_frame_data_ = new int32_t[width * height];

  for (int32_t i = 0; i < width * height; i++) {
    last_frame_data_[i] = TRANSPARENT_COLOR;
  }

  /*
    GIF Header
  */

  // Signature (3bytes)
  // Version (3bytes)
  ofs_.write("GIF89a", 6);

  // Logical Screen Width (2bytes)
  int32_t scaled_width = width * SCREEN_CAPTURE_SCALE;
  ofs_.put(scaled_width & 0xff);
  ofs_.put((scaled_width >> 8) & 0xff);

  // Logical Screen Height (2bytes)
  int32_t scaled_height = height * SCREEN_CAPTURE_SCALE;
  ofs_.put(scaled_height & 0xff);
  ofs_.put((scaled_height >> 8) & 0xff);

  // Global Color Table Flag (1bit)
  // Color Resolution (3bits)
  // Sort Flag (1bit)
  // Size of Global Color Table (3bits)
  ofs_.put(0xc4);

  // Background Color Index (1byte)
  ofs_.put(TRANSPARENT_COLOR);

  // Pixel Aspect Ratio (1byte)
  ofs_.put(0);

  // Global Color Table
  for (int i = 0; i < 16; i++) {
    int32_t color = palette_color[i];
    ofs_.put((color >> 16) & 0xff);
    ofs_.put((color >> 8) & 0xff);
    ofs_.put(color & 0xff);
  }

  for (int i = 0; i < 16; i++) {
    ofs_.put(0);
    ofs_.put(0);
    ofs_.put(0);
  }

  /*
    Application Extension
  */

  // Extension Introducer (1byte)
  ofs_.put(0x21);  // extension

  // Extention Label (1byte)
  ofs_.put(0xff);  // application specific

  // Block Size (1byte)
  ofs_.put(11);  // length 11

  ofs_.write("NETSCAPE2.0", 11);
  ofs_.put(3);  // 3 bytes of NETSCAPE2.0 data

  ofs_.put(1);  // fixed at 1
  ofs_.put(0);  // loop count
  ofs_.put(0);  // loop count

  // Block Terminator (1byte)
  ofs_.put(0);
}

GifWriter::~GifWriter() {
  delete[] last_frame_data_;
}

void GifWriter::AddFrame(const Image* image, int32_t delay_time) {
  /*
    Graphics Control Extension
  */

  // Extension Introducer (1byte)
  ofs_.put(0x21);

  // Graphic Control Label (1byte)
  ofs_.put(0xf9);

  // Block Size (1byte)
  ofs_.put(0x04);

  // Reserved (3bits)
  // Disposal Method (3bits)
  // User Input Flag (1bit)
  // Transparent Color Flag (1bit)
  ofs_.put(0x01);

  // Delay Time (2bytes)
  ofs_.put(delay_time & 0xff);
  ofs_.put((delay_time >> 8) & 0xff);

  // Transparent Color Index (1byte)
  ofs_.put(TRANSPARENT_COLOR);

  // Block Terminator (1byte)
  ofs_.put(0);

  /*
    Image Block
  */

  // Image Separator (1byte)
  ofs_.put(0x2c);

  // Image Left Position (2bytes)
  ofs_.put(0);
  ofs_.put(0);

  // Image Top Position (2bytes)
  ofs_.put(0);
  ofs_.put(0);

  // Image Width (2bytes)
  int32_t scaled_width = width_ * SCREEN_CAPTURE_SCALE;
  ofs_.put(scaled_width & 0xff);
  ofs_.put((scaled_width >> 8) & 0xff);

  // Image Height (2bytes)
  int32_t scaled_height = height_ * SCREEN_CAPTURE_SCALE;
  ofs_.put(scaled_height & 0xff);
  ofs_.put((scaled_height >> 8) & 0xff);

  // Local Color Table Flag (1bit)
  // Interlace Flag (1bit)
  // Sort Flag (1bit)
  // Reserved (2bits)
  // Size of Local Color Table (3bits)
  ofs_.put(0x00);

  // LZW Minimum Code Size (1byte)
  ofs_.put(MIN_CODE_SIZE);

  int32_t** data = image->Data();
  ImageDataBlock block(&ofs_);
  static uint16_t code_tree[MAX_CODE_COUNT][VALUE_TYPE_COUNT];

  int32_t code_size = MIN_CODE_SIZE + 1;
  int32_t code_index = CLEAR_CODE + 1;
  int32_t code = -1;

  block.AddCode(CLEAR_CODE, code_size);
  ClearCodeTree(code_tree);

  for (int32_t i = 0; i < scaled_height; i++) {
    int32_t y = i / SCREEN_CAPTURE_SCALE;

    for (int32_t j = 0; j < scaled_width; j++) {
      int32_t x = j / SCREEN_CAPTURE_SCALE;
      int32_t value = data[y][x];

      if (value == last_frame_data_[width_ * y + x]) {
        value = TRANSPARENT_COLOR;
      }

      if (code >= MAX_CODE_COUNT || code_size > MAX_CODE_SIZE ||
          value >= VALUE_TYPE_COUNT) {
        PYXEL_ERROR("failed to generate GIF");
      }

      if (code < 0) {
        code = value;
      } else if (code_tree[code][value] > 0) {
        code = code_tree[code][value];
      } else {
        block.AddCode(code, code_size);

        code_index++;
        code_tree[code][value] = code_index;

        if (code >= code_index) {
          PYXEL_ERROR("failed to generate GIF");
        }

        if (code_index >= (1 << code_size)) {
          code_size++;
        }

        if (code_index == MAX_CODE_COUNT - 1) {
          block.AddCode(CLEAR_CODE, code_size);
          ClearCodeTree(code_tree);

          code_size = MIN_CODE_SIZE + 1;
          code_index = CLEAR_CODE + 1;
        }

        code = value;
      }
    }
  }

  block.AddCode(code, code_size);
  block.AddCode(CLEAR_CODE, code_size);
  block.AddCode(CLEAR_CODE + 1, MIN_CODE_SIZE + 1);
  block.EndCode();

  // Block Terminator (1byte)
  ofs_.put(0);

  memcpy(last_frame_data_, image->Data()[0],
         sizeof(int32_t) * width_ * height_);
}

void GifWriter::EndFrame() {
  // Trailer (1byte)
  ofs_.put(0x3b);

  ofs_.close();
}

}  // namespace pyxelcore
*/
