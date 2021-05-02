import sdl2

#[
type
  Window* = ref object
    SDL_Window* window_;
    SDL_Renderer* renderer_;
    SDL_Texture* screen_texture_;

    int32_t window_x_;
    int32_t window_y_;
    int32_t screen_x_;
    int32_t screen_y_;
    int32_t screen_width_;
    int32_t screen_height_;
    int32_t screen_scale_;
    PaletteColor palette_color_;
    bool is_fullscreen_;
    int32_t mouse_wheel_;
    std::string drop_file_;


class Window {
 public:
  Window(const std::string& caption,
         int32_t screen_width,
         int32_t screen_height,
         int32_t screen_scale,
         const PaletteColor& palette_color);

  int32_t WindowX() const { return window_x_; }
  int32_t WindowY() const { return window_y_; }
  int32_t ScreenX() const { return screen_x_; }
  int32_t ScreenY() const { return screen_y_; }
  int32_t ScreenWidth() const { return screen_width_; }
  int32_t ScreenHeight() const { return screen_height_; }
  int32_t ScreenScale() const { return screen_scale_; }

  void ToggleFullscreen();
  bool ProcessEvents();
  void Render(int32_t** screen_data);
  int32_t GetMouseWheel();
  std::string GetDropFile();
  void SetCaption(const std::string& caption);

 private:

  void SetupWindowIcon() const;
  void UpdateWindowInfo();
  void UpdateScreenTexture(int32_t** screen_data);
};

]#
