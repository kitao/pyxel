#include "pyxelcore/gif_writer.h"

#include "pyxelcore/image.h"

namespace pyxelcore {

class GifBitStream {
 public:
  GifBitStream(std::ofstream* ofs) {
    ofs_ = ofs;
    bit_index_ = 0;
    bit_data_ = 0;
    chunk_index_ = 0;
  }

  uint8_t BitIndex() { return bit_index_; }
  uint8_t ChunkIndex() { return chunk_index_; }

  void WriteBit(uint32_t bit) {
    bit_data_ |= (bit & 1) << bit_index_;
    bit_index_++;

    if (bit_index_ > 7) {
      chunk_data_[chunk_index_] = bit_data_;

      bit_index_ = 0;
      bit_data_ = 0;
      chunk_index_++;
    }
  }

  void WriteChunk() {
    ofs_->put(chunk_index_);
    ofs_->write(reinterpret_cast<char*>(chunk_data_), chunk_index_);

    bit_index_ = 0;
    bit_data_ = 0;
    chunk_index_ = 0;
  }

  void WriteCode(uint32_t code, int32_t length) {
    for (int32_t i = 0; i < length; i++) {
      WriteBit(code);
      code >>= 1;

      if (chunk_index_ == 255) {
        WriteChunk();
      }
    }
  }

 private:
  std::ofstream* ofs_;
  int32_t bit_index_;
  int32_t bit_data_;
  int32_t chunk_index_;
  uint8_t chunk_data_[256];
};

GifWriter::GifWriter(const std::string& filename,
                     int32_t width,
                     int32_t height,
                     const PaletteColor& palette_color,
                     int32_t delay_time) {
  ofs_ = std::ofstream(filename);
  width_ = width;
  height_ = height;
  delay_time_ = delay_time;

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
  ofs_.put(0xb3);

  // Background Color Index (1byte)
  ofs_.put(0);

  // Pixel Aspect Ratio (1byte)
  ofs_.put(0);

  // Global Color Table
  for (int i = 0; i < 16; i++) {
    int32_t color = palette_color[i];
    ofs_.put((color >> 16) & 0xff);
    ofs_.put((color >> 8) & 0xff);
    ofs_.put(color & 0xff);
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

  ofs_.put(1);
  ofs_.put(0);
  ofs_.put(0);

  // Block Terminator (1byte)
  ofs_.put(0);
}

void GifWriter::AddFrame(const Image* image) {
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
  ofs_.put(0 /*0x05*/);

  // Delay Time (2bytes)
  ofs_.put(delay_time_ & 0xff);
  ofs_.put((delay_time_ >> 8) & 0xff);

  // Transparent Color Index (1byte)
  ofs_.put(0 /*16*/);

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
  ofs_.put(0);

  // LZW Minimum Code Size (1byte)
  const int MIN_CODE_SIZE = 5;
  ofs_.put(MIN_CODE_SIZE);

  struct GifLzwNode {
    uint16_t next[256];
  };

  const int32_t MAX_CODE_COUNT = 4096;
  GifLzwNode* code_tree = new GifLzwNode[MAX_CODE_COUNT];
  memset(code_tree, 0, sizeof(GifLzwNode) * MAX_CODE_COUNT);

  int32_t clear_code = 1 << MIN_CODE_SIZE;
  int32_t code_size = MIN_CODE_SIZE + 1;
  int32_t max_code_index = clear_code + 1;
  int32_t code = -1;

  GifBitStream bs(&ofs_);
  int32_t** data = image->Data();

  bs.WriteCode(clear_code, code_size);

  for (int32_t i = 0; i < scaled_height; i++) {
    for (int32_t j = 0; j < scaled_width; j++) {
      int32_t value = data[i / SCREEN_CAPTURE_SCALE][j / SCREEN_CAPTURE_SCALE];

      if (code < 0) {
        code = value;
      } else if (code_tree[code].next[value]) {
        code = code_tree[code].next[value];
      } else {
        bs.WriteCode(code, code_size);

        max_code_index++;
        code_tree[code].next[value] = max_code_index;

        if (max_code_index >= (1ul << code_size)) {
          code_size++;
        }

        if (max_code_index == MAX_CODE_COUNT - 1) {
          bs.WriteCode(clear_code, code_size);

          memset(code_tree, 0, sizeof(GifLzwNode) * MAX_CODE_COUNT);
          code_size = MIN_CODE_SIZE + 1;
          max_code_index = clear_code + 1;
        }

        code = value;
      }
    }
  }

  bs.WriteCode(code, code_size);
  bs.WriteCode(clear_code, code_size);
  bs.WriteCode(clear_code + 1, MIN_CODE_SIZE + 1);

  while (bs.BitIndex() > 0) {
    bs.WriteBit(0);
  }

  if (bs.ChunkIndex() > 0) {
    bs.WriteChunk();
  }

  // Block Terminator (1byte)
  ofs_.put(0);

  delete[] code_tree;
}

void GifWriter::EndFrame() {
  // Trailer (1byte)
  ofs_.put(0x3b);

  ofs_.close();
}

}  // namespace pyxelcore