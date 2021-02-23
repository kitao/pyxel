global_instance!(Graphics, graphics);

pub struct Graphics {
    screen_width: usize,
    screen_height: usize,
    /*
    private:
        Image** image_bank_;
        Tilemap** tilemap_bank_;
        int32_t screen_width_;
        int32_t screen_height_;
        int32_t** screen_data_;
        Rectangle clip_area_;
        pyxelcore::PaletteTable palette_table_;

        void SetupMouseCursor();
        void SetupFont();
        int32_t GetDrawColor(int32_t color, const std::string& func_name) const;
        void SetPixel(int32_t x, int32_t y, int32_t color);
    */
}

pub fn init_graphics(screen_width: usize, screen_height: usize) {
    let graphics = Graphics {
        screen_width: screen_width,
        screen_height: screen_height,
    };

    set_instance(graphics);
}

impl Graphics {
    pub fn screen_size(&self) -> (usize, usize) {
        (self.screen_width, self.screen_height)
    }

    /*
    public:
    Graphics(int32_t width, int32_t height);
    ~Graphics();

    const Rectangle& ClipArea() const { return clip_area_; }
    const pyxelcore::PaletteTable& PaletteTable() const { return palette_table_; }
    Image* ScreenImage() const { return image_bank_[IMAGE_BANK_FOR_SCREEN]; }

    Image* GetImageBank(int32_t image_index, bool system = false) const;
    Tilemap* GetTilemapBank(int32_t tilemap_index) const;

    void ResetClipArea();
    void SetClipArea(int32_t x, int32_t y, int32_t width, int32_t height);
    void ResetPalette();
    void SetPalette(int32_t src_color, int32_t dst_color);
    void ClearScreen(int32_t color);
    int32_t GetPoint(int32_t x, int32_t y);
    void SetPoint(int32_t x, int32_t y, int32_t color);
    void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t color);
    void DrawRectangle(int32_t x,
                        int32_t y,
                        int32_t width,
                        int32_t height,
                        int32_t color);
    void DrawRectangleBorder(int32_t x,
                            int32_t y,
                            int32_t width,
                            int32_t height,
                            int32_t color);
    void DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color);
    void DrawCircleBorder(int32_t x, int32_t y, int32_t radius, int32_t color);
    void DrawTriangle(int32_t x1,
                    int32_t y1,
                    int32_t x2,
                    int32_t y2,
                    int32_t x3,
                    int32_t y3,
                    int32_t color);
    void DrawTriangleBorder(int32_t x1,
                            int32_t y1,
                            int32_t x2,
                            int32_t y2,
                            int32_t x3,
                            int32_t y3,
                            int32_t color);
    void DrawImage(int32_t x,
                    int32_t y,
                    int32_t image_index,
                    int32_t u,
                    int32_t v,
                    int32_t width,
                    int32_t height,
                    int32_t color_key = -1);
    void DrawTilemap(int32_t x,
                    int32_t y,
                    int32_t tilemap_index,
                    int32_t u,
                    int32_t v,
                    int32_t width,
                    int32_t height,
                    int32_t color_key = -1);
    void DrawText(int32_t x, int32_t y, const char* text, int32_t color);
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_screen_size() {
        crate::init_graphics(100, 200);
        assert_eq!(crate::graphics().screen_size(), (100, 200));
    }
}
