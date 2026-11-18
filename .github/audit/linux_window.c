#include <SDL2/SDL.h>
#include <SDL2/SDL_syswm.h>

unsigned long current_x11_window(void)
{
    SDL_SysWMinfo info;
    SDL_VERSION(&info.version);
    SDL_Window *window = SDL_GL_GetCurrentWindow();
    if (!window || !SDL_GetWindowWMInfo(window, &info) || info.subsystem != SDL_SYSWM_X11)
        return 0;
    return info.info.x11.window;
}
