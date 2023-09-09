use paste::paste;

use crate::sdl2_sys::*;

pub type Key = u32;
pub type KeyValue = i32;

// Unknown
pub const KEY_UNKNOWN: Key = SDLK_UNKNOWN;

// Keyboard
pub const KEY_RETURN: Key = SDLK_RETURN;
pub const KEY_ESCAPE: Key = SDLK_ESCAPE;
pub const KEY_BACKSPACE: Key = SDLK_BACKSPACE;
pub const KEY_TAB: Key = SDLK_TAB;
pub const KEY_SPACE: Key = SDLK_SPACE;
pub const KEY_EXCLAIM: Key = SDLK_EXCLAIM;
pub const KEY_QUOTEDBL: Key = SDLK_QUOTEDBL;
pub const KEY_HASH: Key = SDLK_HASH;
pub const KEY_PERCENT: Key = SDLK_PERCENT;
pub const KEY_DOLLAR: Key = SDLK_DOLLAR;
pub const KEY_AMPERSAND: Key = SDLK_AMPERSAND;
pub const KEY_QUOTE: Key = SDLK_QUOTE;
pub const KEY_LEFTPAREN: Key = SDLK_LEFTPAREN;
pub const KEY_RIGHTPAREN: Key = SDLK_RIGHTPAREN;
pub const KEY_ASTERISK: Key = SDLK_ASTERISK;
pub const KEY_PLUS: Key = SDLK_PLUS;
pub const KEY_COMMA: Key = SDLK_COMMA;
pub const KEY_MINUS: Key = SDLK_MINUS;
pub const KEY_PERIOD: Key = SDLK_PERIOD;
pub const KEY_SLASH: Key = SDLK_SLASH;
pub const KEY_0: Key = SDLK_0;
pub const KEY_1: Key = SDLK_1;
pub const KEY_2: Key = SDLK_2;
pub const KEY_3: Key = SDLK_3;
pub const KEY_4: Key = SDLK_4;
pub const KEY_5: Key = SDLK_5;
pub const KEY_6: Key = SDLK_6;
pub const KEY_7: Key = SDLK_7;
pub const KEY_8: Key = SDLK_8;
pub const KEY_9: Key = SDLK_9;
pub const KEY_COLON: Key = SDLK_COLON;
pub const KEY_SEMICOLON: Key = SDLK_SEMICOLON;
pub const KEY_LESS: Key = SDLK_LESS;
pub const KEY_EQUALS: Key = SDLK_EQUALS;
pub const KEY_GREATER: Key = SDLK_GREATER;
pub const KEY_QUESTION: Key = SDLK_QUESTION;
pub const KEY_AT: Key = SDLK_AT;

pub const KEY_LEFTBRACKET: Key = SDLK_LEFTBRACKET;
pub const KEY_BACKSLASH: Key = SDLK_BACKSLASH;
pub const KEY_RIGHTBRACKET: Key = SDLK_RIGHTBRACKET;
pub const KEY_CARET: Key = SDLK_CARET;
pub const KEY_UNDERSCORE: Key = SDLK_UNDERSCORE;
pub const KEY_BACKQUOTE: Key = SDLK_BACKQUOTE;
pub const KEY_A: Key = SDLK_a;
pub const KEY_B: Key = SDLK_b;
pub const KEY_C: Key = SDLK_c;
pub const KEY_D: Key = SDLK_d;
pub const KEY_E: Key = SDLK_e;
pub const KEY_F: Key = SDLK_f;
pub const KEY_G: Key = SDLK_g;
pub const KEY_H: Key = SDLK_h;
pub const KEY_I: Key = SDLK_i;
pub const KEY_J: Key = SDLK_j;
pub const KEY_K: Key = SDLK_k;
pub const KEY_L: Key = SDLK_l;
pub const KEY_M: Key = SDLK_m;
pub const KEY_N: Key = SDLK_n;
pub const KEY_O: Key = SDLK_o;
pub const KEY_P: Key = SDLK_p;
pub const KEY_Q: Key = SDLK_q;
pub const KEY_R: Key = SDLK_r;
pub const KEY_S: Key = SDLK_s;
pub const KEY_T: Key = SDLK_t;
pub const KEY_U: Key = SDLK_u;
pub const KEY_V: Key = SDLK_v;
pub const KEY_W: Key = SDLK_w;
pub const KEY_X: Key = SDLK_x;
pub const KEY_Y: Key = SDLK_y;
pub const KEY_Z: Key = SDLK_z;

pub const KEY_CAPSLOCK: Key = SDLK_CAPSLOCK;

pub const KEY_F1: Key = SDLK_F1;
pub const KEY_F2: Key = SDLK_F2;
pub const KEY_F3: Key = SDLK_F3;
pub const KEY_F4: Key = SDLK_F4;
pub const KEY_F5: Key = SDLK_F5;
pub const KEY_F6: Key = SDLK_F6;
pub const KEY_F7: Key = SDLK_F7;
pub const KEY_F8: Key = SDLK_F8;
pub const KEY_F9: Key = SDLK_F9;
pub const KEY_F10: Key = SDLK_F10;
pub const KEY_F11: Key = SDLK_F11;
pub const KEY_F12: Key = SDLK_F12;

pub const KEY_PRINTSCREEN: Key = SDLK_PRINTSCREEN;
pub const KEY_SCROLLLOCK: Key = SDLK_SCROLLLOCK;
pub const KEY_PAUSE: Key = SDLK_PAUSE;
pub const KEY_INSERT: Key = SDLK_INSERT;
pub const KEY_HOME: Key = SDLK_HOME;
pub const KEY_PAGEUP: Key = SDLK_PAGEUP;
pub const KEY_DELETE: Key = SDLK_DELETE;
pub const KEY_END: Key = SDLK_END;
pub const KEY_PAGEDOWN: Key = SDLK_PAGEDOWN;
pub const KEY_RIGHT: Key = SDLK_RIGHT;
pub const KEY_LEFT: Key = SDLK_LEFT;
pub const KEY_DOWN: Key = SDLK_DOWN;
pub const KEY_UP: Key = SDLK_UP;

pub const KEY_NUMLOCKCLEAR: Key = SDLK_NUMLOCKCLEAR;
pub const KEY_KP_DIVIDE: Key = SDLK_KP_DIVIDE;
pub const KEY_KP_MULTIPLY: Key = SDLK_KP_MULTIPLY;
pub const KEY_KP_MINUS: Key = SDLK_KP_MINUS;
pub const KEY_KP_PLUS: Key = SDLK_KP_PLUS;
pub const KEY_KP_ENTER: Key = SDLK_KP_ENTER;
pub const KEY_KP_1: Key = SDLK_KP_1;
pub const KEY_KP_2: Key = SDLK_KP_2;
pub const KEY_KP_3: Key = SDLK_KP_3;
pub const KEY_KP_4: Key = SDLK_KP_4;
pub const KEY_KP_5: Key = SDLK_KP_5;
pub const KEY_KP_6: Key = SDLK_KP_6;
pub const KEY_KP_7: Key = SDLK_KP_7;
pub const KEY_KP_8: Key = SDLK_KP_8;
pub const KEY_KP_9: Key = SDLK_KP_9;
pub const KEY_KP_0: Key = SDLK_KP_0;
pub const KEY_KP_PERIOD: Key = SDLK_KP_PERIOD;

pub const KEY_APPLICATION: Key = SDLK_APPLICATION;
pub const KEY_POWER: Key = SDLK_POWER;
pub const KEY_KP_EQUALS: Key = SDLK_KP_EQUALS;
pub const KEY_F13: Key = SDLK_F13;
pub const KEY_F14: Key = SDLK_F14;
pub const KEY_F15: Key = SDLK_F15;
pub const KEY_F16: Key = SDLK_F16;
pub const KEY_F17: Key = SDLK_F17;
pub const KEY_F18: Key = SDLK_F18;
pub const KEY_F19: Key = SDLK_F19;
pub const KEY_F20: Key = SDLK_F20;
pub const KEY_F21: Key = SDLK_F21;
pub const KEY_F22: Key = SDLK_F22;
pub const KEY_F23: Key = SDLK_F23;
pub const KEY_F24: Key = SDLK_F24;
pub const KEY_EXECUTE: Key = SDLK_EXECUTE;
pub const KEY_HELP: Key = SDLK_HELP;
pub const KEY_MENU: Key = SDLK_MENU;
pub const KEY_SELECT: Key = SDLK_SELECT;
pub const KEY_STOP: Key = SDLK_STOP;
pub const KEY_AGAIN: Key = SDLK_AGAIN;
pub const KEY_UNDO: Key = SDLK_UNDO;
pub const KEY_CUT: Key = SDLK_CUT;
pub const KEY_COPY: Key = SDLK_COPY;
pub const KEY_PASTE: Key = SDLK_PASTE;
pub const KEY_FIND: Key = SDLK_FIND;
pub const KEY_MUTE: Key = SDLK_MUTE;
pub const KEY_VOLUMEUP: Key = SDLK_VOLUMEUP;
pub const KEY_VOLUMEDOWN: Key = SDLK_VOLUMEDOWN;
pub const KEY_KP_COMMA: Key = SDLK_KP_COMMA;
pub const KEY_KP_EQUALSAS400: Key = SDLK_KP_EQUALSAS400;

pub const KEY_ALTERASE: Key = SDLK_ALTERASE;
pub const KEY_SYSREQ: Key = SDLK_SYSREQ;
pub const KEY_CANCEL: Key = SDLK_CANCEL;
pub const KEY_CLEAR: Key = SDLK_CLEAR;
pub const KEY_PRIOR: Key = SDLK_PRIOR;
pub const KEY_RETURN2: Key = SDLK_RETURN2;
pub const KEY_SEPARATOR: Key = SDLK_SEPARATOR;
pub const KEY_OUT: Key = SDLK_OUT;
pub const KEY_OPER: Key = SDLK_OPER;
pub const KEY_CLEARAGAIN: Key = SDLK_CLEARAGAIN;
pub const KEY_CRSEL: Key = SDLK_CRSEL;
pub const KEY_EXSEL: Key = SDLK_EXSEL;

pub const KEY_KP_00: Key = SDLK_KP_00;
pub const KEY_KP_000: Key = SDLK_KP_000;
pub const KEY_THOUSANDSSEPARATOR: Key = SDLK_THOUSANDSSEPARATOR;
pub const KEY_DECIMALSEPARATOR: Key = SDLK_DECIMALSEPARATOR;
pub const KEY_CURRENCYUNIT: Key = SDLK_CURRENCYUNIT;
pub const KEY_CURRENCYSUBUNIT: Key = SDLK_CURRENCYSUBUNIT;
pub const KEY_KP_LEFTPAREN: Key = SDLK_KP_LEFTPAREN;
pub const KEY_KP_RIGHTPAREN: Key = SDLK_KP_RIGHTPAREN;
pub const KEY_KP_LEFTBRACE: Key = SDLK_KP_LEFTBRACE;
pub const KEY_KP_RIGHTBRACE: Key = SDLK_KP_RIGHTBRACE;
pub const KEY_KP_TAB: Key = SDLK_KP_TAB;
pub const KEY_KP_BACKSPACE: Key = SDLK_KP_BACKSPACE;
pub const KEY_KP_A: Key = SDLK_KP_A;
pub const KEY_KP_B: Key = SDLK_KP_B;
pub const KEY_KP_C: Key = SDLK_KP_C;
pub const KEY_KP_D: Key = SDLK_KP_D;
pub const KEY_KP_E: Key = SDLK_KP_E;
pub const KEY_KP_F: Key = SDLK_KP_F;
pub const KEY_KP_XOR: Key = SDLK_KP_XOR;
pub const KEY_KP_POWER: Key = SDLK_KP_POWER;
pub const KEY_KP_PERCENT: Key = SDLK_KP_PERCENT;
pub const KEY_KP_LESS: Key = SDLK_KP_LESS;
pub const KEY_KP_GREATER: Key = SDLK_KP_GREATER;
pub const KEY_KP_AMPERSAND: Key = SDLK_KP_AMPERSAND;
pub const KEY_KP_DBLAMPERSAND: Key = SDLK_KP_DBLAMPERSAND;
pub const KEY_KP_VERTICALBAR: Key = SDLK_KP_VERTICALBAR;
pub const KEY_KP_DBLVERTICALBAR: Key = SDLK_KP_DBLVERTICALBAR;
pub const KEY_KP_COLON: Key = SDLK_KP_COLON;
pub const KEY_KP_HASH: Key = SDLK_KP_HASH;
pub const KEY_KP_SPACE: Key = SDLK_KP_SPACE;
pub const KEY_KP_AT: Key = SDLK_KP_AT;
pub const KEY_KP_EXCLAM: Key = SDLK_KP_EXCLAM;
pub const KEY_KP_MEMSTORE: Key = SDLK_KP_MEMSTORE;
pub const KEY_KP_MEMRECALL: Key = SDLK_KP_MEMRECALL;
pub const KEY_KP_MEMCLEAR: Key = SDLK_KP_MEMCLEAR;
pub const KEY_KP_MEMADD: Key = SDLK_KP_MEMADD;
pub const KEY_KP_MEMSUBTRACT: Key = SDLK_KP_MEMSUBTRACT;
pub const KEY_KP_MEMMULTIPLY: Key = SDLK_KP_MEMMULTIPLY;
pub const KEY_KP_MEMDIVIDE: Key = SDLK_KP_MEMDIVIDE;
pub const KEY_KP_PLUSMINUS: Key = SDLK_KP_PLUSMINUS;
pub const KEY_KP_CLEAR: Key = SDLK_KP_CLEAR;
pub const KEY_KP_CLEARENTRY: Key = SDLK_KP_CLEARENTRY;
pub const KEY_KP_BINARY: Key = SDLK_KP_BINARY;
pub const KEY_KP_OCTAL: Key = SDLK_KP_OCTAL;
pub const KEY_KP_DECIMAL: Key = SDLK_KP_DECIMAL;
pub const KEY_KP_HEXADECIMAL: Key = SDLK_KP_HEXADECIMAL;

pub const KEY_LCTRL: Key = SDLK_LCTRL;
pub const KEY_LSHIFT: Key = SDLK_LSHIFT;
pub const KEY_LALT: Key = SDLK_LALT;
pub const KEY_LGUI: Key = SDLK_LGUI;
pub const KEY_RCTRL: Key = SDLK_RCTRL;
pub const KEY_RSHIFT: Key = SDLK_RSHIFT;
pub const KEY_RALT: Key = SDLK_RALT;
pub const KEY_RGUI: Key = SDLK_RGUI;

pub const KEY_MODE: Key = SDLK_MODE;

pub const KEY_AUDIONEXT: Key = SDLK_AUDIONEXT;
pub const KEY_AUDIOPREV: Key = SDLK_AUDIOPREV;
pub const KEY_AUDIOSTOP: Key = SDLK_AUDIOSTOP;
pub const KEY_AUDIOPLAY: Key = SDLK_AUDIOPLAY;
pub const KEY_AUDIOMUTE: Key = SDLK_AUDIOMUTE;
pub const KEY_MEDIASELECT: Key = SDLK_MEDIASELECT;
pub const KEY_WWW: Key = SDLK_WWW;
pub const KEY_MAIL: Key = SDLK_MAIL;
pub const KEY_CALCULATOR: Key = SDLK_CALCULATOR;
pub const KEY_COMPUTER: Key = SDLK_COMPUTER;
pub const KEY_AC_SEARCH: Key = SDLK_AC_SEARCH;
pub const KEY_AC_HOME: Key = SDLK_AC_HOME;
pub const KEY_AC_BACK: Key = SDLK_AC_BACK;
pub const KEY_AC_FORWARD: Key = SDLK_AC_FORWARD;
pub const KEY_AC_STOP: Key = SDLK_AC_STOP;
pub const KEY_AC_REFRESH: Key = SDLK_AC_REFRESH;
pub const KEY_AC_BOOKMARKS: Key = SDLK_AC_BOOKMARKS;

pub const KEY_BRIGHTNESSDOWN: Key = SDLK_BRIGHTNESSDOWN;
pub const KEY_BRIGHTNESSUP: Key = SDLK_BRIGHTNESSUP;
pub const KEY_DISPLAYSWITCH: Key = SDLK_DISPLAYSWITCH;
pub const KEY_KBDILLUMTOGGLE: Key = SDLK_KBDILLUMTOGGLE;
pub const KEY_KBDILLUMDOWN: Key = SDLK_KBDILLUMDOWN;
pub const KEY_KBDILLUMUP: Key = SDLK_KBDILLUMUP;
pub const KEY_EJECT: Key = SDLK_EJECT;
pub const KEY_SLEEP: Key = SDLK_SLEEP;
pub const KEY_APP1: Key = SDLK_APP1;
pub const KEY_APP2: Key = SDLK_APP2;

pub const KEY_AUDIOREWIND: Key = SDLK_AUDIOREWIND;
pub const KEY_AUDIOFASTFORWARD: Key = SDLK_AUDIOFASTFORWARD;

pub const KEY_SOFTLEFT: Key = SDLK_SOFTLEFT;
pub const KEY_SOFTRIGHT: Key = SDLK_SOFTRIGHT;
pub const KEY_CALL: Key = SDLK_CALL;
pub const KEY_ENDCALL: Key = SDLK_ENDCALL;

pub const SPECIAL_KEY_START_INDEX: Key = 10000;
pub const KEY_NONE: Key = SPECIAL_KEY_START_INDEX + 0;
pub const KEY_SHIFT: Key = SPECIAL_KEY_START_INDEX + 1;
pub const KEY_CTRL: Key = SPECIAL_KEY_START_INDEX + 2;
pub const KEY_ALT: Key = SPECIAL_KEY_START_INDEX + 3;
pub const KEY_GUI: Key = SPECIAL_KEY_START_INDEX + 4;

// Mouse
pub const MOUSE_KEY_START_INDEX: Key = 11000;
pub const MOUSE_POS_X: Key = MOUSE_KEY_START_INDEX + 0;
pub const MOUSE_POS_Y: Key = MOUSE_KEY_START_INDEX + 1;
pub const MOUSE_WHEEL_X: Key = MOUSE_KEY_START_INDEX + 2;
pub const MOUSE_WHEEL_Y: Key = MOUSE_KEY_START_INDEX + 3;
pub const MOUSE_BUTTON_LEFT: Key = MOUSE_KEY_START_INDEX + 4;
pub const MOUSE_BUTTON_MIDDLE: Key = MOUSE_KEY_START_INDEX + 5;
pub const MOUSE_BUTTON_RIGHT: Key = MOUSE_KEY_START_INDEX + 6;
pub const MOUSE_BUTTON_X1: Key = MOUSE_KEY_START_INDEX + 7;
pub const MOUSE_BUTTON_X2: Key = MOUSE_KEY_START_INDEX + 8;

// Gamepad
pub const GAMEPAD_KEY_START_INDEX: Key = 12000;
pub const GAMEPAD_KEY_INDEX_INTERVAL: Key = 1000;

macro_rules! define_gamepad_keys {
    ($gamepad_name:ident, $start_index:expr) => {
        paste! {
            pub const [<$gamepad_name _AXIS_LEFTX>]: Key  = $start_index + 0;
            pub const [<$gamepad_name _AXIS_LEFTY>]: Key  = $start_index + 1;
            pub const [<$gamepad_name _AXIS_RIGHTX>]: Key  = $start_index + 2;
            pub const [<$gamepad_name _AXIS_RIGHTY>]: Key  = $start_index + 3;
            pub const [<$gamepad_name _AXIS_TRIGGERLEFT>]: Key  = $start_index + 4;
            pub const [<$gamepad_name _AXIS_TRIGGERRIGHT>]: Key  = $start_index + 5;
            pub const [<$gamepad_name _BUTTON_A>]: Key  = $start_index + 6;
            pub const [<$gamepad_name _BUTTON_B>]: Key  = $start_index + 7;
            pub const [<$gamepad_name _BUTTON_X>]: Key  = $start_index + 8;
            pub const [<$gamepad_name _BUTTON_Y>]: Key  = $start_index + 9;
            pub const [<$gamepad_name _BUTTON_BACK>]: Key  = $start_index + 10;
            pub const [<$gamepad_name _BUTTON_GUIDE>]: Key  = $start_index + 11;
            pub const [<$gamepad_name _BUTTON_START>]: Key  = $start_index + 12;
            pub const [<$gamepad_name _BUTTON_LEFTSTICK>]: Key  = $start_index + 13;
            pub const [<$gamepad_name _BUTTON_RIGHTSTICK>]: Key  = $start_index + 14;
            pub const [<$gamepad_name _BUTTON_LEFTSHOULDER>]: Key  = $start_index + 15;
            pub const [<$gamepad_name _BUTTON_RIGHTSHOULDER>]: Key  = $start_index + 16;
            pub const [<$gamepad_name _BUTTON_DPAD_UP>]: Key  = $start_index + 17;
            pub const [<$gamepad_name _BUTTON_DPAD_DOWN>]: Key  = $start_index + 18;
            pub const [<$gamepad_name _BUTTON_DPAD_LEFT>]: Key  = $start_index + 19;
            pub const [<$gamepad_name _BUTTON_DPAD_RIGHT>]: Key  = $start_index + 20;
            pub const [<$gamepad_name _BUTTON_MISC1>]: Key  = $start_index + 21;
            pub const [<$gamepad_name _BUTTON_PADDLE1>]: Key  = $start_index + 22;
            pub const [<$gamepad_name _BUTTON_PADDLE2>]: Key  = $start_index + 23;
            pub const [<$gamepad_name _BUTTON_PADDLE3>]: Key  = $start_index + 24;
            pub const [<$gamepad_name _BUTTON_PADDLE4>]: Key  = $start_index + 25;
            pub const [<$gamepad_name _BUTTON_TOUCHPAD>]: Key  = $start_index + 26;
        }
    };
}

define_gamepad_keys!(
    GAMEPAD1,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 0
);
define_gamepad_keys!(
    GAMEPAD2,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 1
);
define_gamepad_keys!(
    GAMEPAD3,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 2
);
define_gamepad_keys!(
    GAMEPAD4,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 3
);
