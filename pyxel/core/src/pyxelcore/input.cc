#include "pyxelcore/input.h"

#include "pyxelcore/window.h"

namespace pyxelcore {

Input::Input() {
  is_mouse_visible_ = false;

  for (int32_t i = 0; i < KEY_COUNT; i++) {
    key_state_[i] = 0;
  }

  /*
    if (!SDL_IsGameController(JoystickIndex))

    ControllerHandles[ControllerIndex] = SDL_GameControllerOpen(JoystickIndex);

    if(ControllerHandles[ControllerIndex] != 0 &&
    SDL_GameControllerGetAttached(ControllerHandles[ControllerIndex]))
    {
        // NOTE: We have a controller with index ControllerIndex.
        bool Up =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_DPAD_UP); bool Down =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_DPAD_DOWN); bool Left =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_DPAD_LEFT); bool Right =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_DPAD_RIGHT); bool Start =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_START); bool Back =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_BACK); bool LeftShoulder =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_LEFTSHOULDER); bool RightShoulder =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_RIGHTSHOULDER); bool AButton =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_A); bool BButton =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_B); bool XButton =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_X); bool YButton =
    SDL_GameControllerGetButton(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_BUTTON_Y);

        int16 StickX =
    SDL_GameControllerGetAxis(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_AXIS_LEFTX); int16 StickY =
    SDL_GameControllerGetAxis(ControllerHandles[ControllerIndex],
    SDL_CONTROLLER_AXIS_LEFTY);
    }
  */
}

Input::~Input() {
  /*
   if (ControllerHandles[ControllerIndex])
   {
       SDL_GameControllerClose(ControllerHandles[ControllerIndex]);
   }
  */
}

void Input::Update(const Window* window, int32_t frame_count) {
  frame_count_ = frame_count + 1;  // change frame_count to start from 1

  SDL_GetGlobalMouseState(&mouse_x_, &mouse_y_);

  mouse_x_ = (mouse_x_ - (window->WindowX() + window->ScreenX())) /
             window->ScreenScale();
  mouse_y_ = (mouse_y_ - (window->WindowY() + window->ScreenY())) /
             window->ScreenScale();

  const uint8_t* sdl_key_state = SDL_GetKeyboardState(NULL);

  for (int32_t i = 0; i < SDL_KEY_COUNT; i++) {
    UpdateKeyState(i, sdl_key_state[SCANCODE_TABLE[i]]);
  }

  UpdateKeyState(KEY_SHIFT, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_SHIFT]] ||
                                sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_SHIFT]]);

  UpdateKeyState(KEY_CONTROL,
                 sdl_key_state[SCANCODE_TABLE[KEY_LEFT_CONTROL]] ||
                     sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_CONTROL]]);

  UpdateKeyState(KEY_ALT, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_ALT]] ||
                              sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_ALT]]);

  UpdateKeyState(KEY_SUPER, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_SUPER]] ||
                                sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_SUPER]]);

  uint32_t mouse_state = SDL_GetMouseState(NULL, NULL);

  UpdateKeyState(MOUSE_LEFT_BUTTON, mouse_state & SDL_BUTTON_LMASK);
  UpdateKeyState(MOUSE_MIDDLE_BUTTON, mouse_state & SDL_BUTTON_MMASK);
  UpdateKeyState(MOUSE_RIGHT_BUTTON, mouse_state & SDL_BUTTON_RMASK);
}

bool Input::IsButtonOn(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    PRINT_ERROR("invalid key");
    return false;
  }

  return key_state_[key] > 0;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) const {
  if (key < 0 || key >= KEY_COUNT) {
    PRINT_ERROR("invalid key");
    return false;
  }

  int32_t press_frame = key_state_[key];

  if (press_frame == frame_count_) {
    return true;
  }

  if (press_frame <= 0 || period_frame <= 0) {
    return false;
  }

  int32_t elapsed_frame = frame_count_ - (press_frame + hold_frame);

  if (elapsed_frame >= 0 && elapsed_frame % period_frame == 0) {
    return true;
  }

  return false;
}

bool Input::IsButtonReleased(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    PRINT_ERROR("invalid key");
    return false;
  }

  return key_state_[key] == -frame_count_;
}

void Input::SetMouseVisible(int32_t is_visible) {
  is_mouse_visible_ = is_visible;
}

/*
    def _update_gamepad(self):
        for i in range(2):
            if i == 0:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_1)
                offset = pyxel.GAMEPAD_1_A
            else:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_2)
                offset = pyxel.GAMEPAD_2_A

            for j in range(count):
                action = states[j]
                button = offset + j

                if action == glfw.PRESS:
                    self._key_state[button] = pyxel.frame_count
                elif action == glfw.RELEASE:
                    if self._key_state.get(button) == pyxel.frame_count:
                        self._key_state[button] = -pyxel.frame_count - 1
                    else:
                        self._key_state[button] = -pyxel.frame_count
*/

}  // namespace pyxelcore
