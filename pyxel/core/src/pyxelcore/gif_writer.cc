#include "pyxelcore/gif_writer.h"

#include "pyxelcore/image.h"

namespace pyxelcore {

template <class T, std::size_t N1, std::size_t N2>
void ClearCodeTree(T (&code_tree)[N1][N2]) {
  for (int32_t i = 0; i < N1; i++) {
    for (int32_t j = 0; j < N2; j++) {
      code_tree[i][j] = -1;
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

  void AddCode(int32_t code, int32_t bit_length) {
    for (int32_t i = 0; i < bit_length; i++) {
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
  uint8_t block_data_[255];

  void WriteBit(int32_t bit) {
    bit_data_ |= (bit & 1) << bit_index_;
    bit_index_++;

    if (bit_index_ == 8) {
      block_data_[block_size_] = bit_data_;
      block_size_++;

      bit_index_ = 0;
      bit_data_ = 0;

      if (block_size_ == 255) {
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
  ofs_ = std::ofstream(filename, std::ios_base::binary);
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

  const int32_t MIN_CODE_SIZE = 5;
  const int32_t MAX_CODE_COUNT = 4096;
  const int32_t CLEAR_CODE = 1 << MIN_CODE_SIZE;

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
  int32_t code_tree[MAX_CODE_COUNT][256];
  ImageDataBlock block(&ofs_);

  int32_t code_size = MIN_CODE_SIZE + 1;
  int32_t code_index = CLEAR_CODE + 1;
  int32_t code = -1;

  block.AddCode(CLEAR_CODE, code_size);
  ClearCodeTree(code_tree);

  for (int32_t i = 0; i < scaled_height; i++) {
    int32_t y = i / SCREEN_CAPTURE_SCALE;

    for (int32_t j = 0; j < scaled_width; j++) {
      int32_t x = j / SCREEN_CAPTURE_SCALE;
      uint8_t value = data[y][x];

      if (value == last_frame_data_[width_ * y + x]) {
        value = TRANSPARENT_COLOR;
      }

      if (code < 0) {
        code = value;
      } else if (code_tree[code][value] >= 0) {
        code = code_tree[code][value];
      } else {
        block.AddCode(code, code_size);

        code_index++;
        code_tree[code][value] = code_index;

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