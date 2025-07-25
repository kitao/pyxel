use paste::paste;

pub type Key = u32;
pub type KeyValue = i32;

// Keyboard (https://wiki.libsdl.org/SDL2/SDLKeycodeLookup)
pub const KEY_BACKSPACE: Key = 0x08;
pub const KEY_TAB: Key = 0x09;
pub const KEY_RETURN: Key = 0x0D;
pub const KEY_ESCAPE: Key = 0x1B;
pub const KEY_SPACE: Key = 0x20;
pub const KEY_EXCLAIM: Key = 0x21;
pub const KEY_QUOTEDBL: Key = 0x22;
pub const KEY_HASH: Key = 0x23;
pub const KEY_DOLLAR: Key = 0x24;
pub const KEY_PERCENT: Key = 0x25;
pub const KEY_AMPERSAND: Key = 0x26;
pub const KEY_QUOTE: Key = 0x27;
pub const KEY_LEFTPAREN: Key = 0x28;
pub const KEY_RIGHTPAREN: Key = 0x29;
pub const KEY_ASTERISK: Key = 0x2A;
pub const KEY_PLUS: Key = 0x2B;
pub const KEY_COMMA: Key = 0x2C;
pub const KEY_MINUS: Key = 0x2D;
pub const KEY_PERIOD: Key = 0x2E;
pub const KEY_SLASH: Key = 0x2F;
pub const KEY_0: Key = 0x30;
pub const KEY_1: Key = 0x31;
pub const KEY_2: Key = 0x32;
pub const KEY_3: Key = 0x33;
pub const KEY_4: Key = 0x34;
pub const KEY_5: Key = 0x35;
pub const KEY_6: Key = 0x36;
pub const KEY_7: Key = 0x37;
pub const KEY_8: Key = 0x38;
pub const KEY_9: Key = 0x39;
pub const KEY_COLON: Key = 0x3A;
pub const KEY_SEMICOLON: Key = 0x3B;
pub const KEY_LESS: Key = 0x3C;
pub const KEY_EQUALS: Key = 0x3D;
pub const KEY_GREATER: Key = 0x3E;
pub const KEY_QUESTION: Key = 0x3F;
pub const KEY_AT: Key = 0x40;
pub const KEY_LEFTBRACKET: Key = 0x5B;
pub const KEY_BACKSLASH: Key = 0x5C;
pub const KEY_RIGHTBRACKET: Key = 0x5D;
pub const KEY_CARET: Key = 0x5E;
pub const KEY_UNDERSCORE: Key = 0x5F;
pub const KEY_BACKQUOTE: Key = 0x60;
pub const KEY_A: Key = 0x61;
pub const KEY_B: Key = 0x62;
pub const KEY_C: Key = 0x63;
pub const KEY_D: Key = 0x64;
pub const KEY_E: Key = 0x65;
pub const KEY_F: Key = 0x66;
pub const KEY_G: Key = 0x67;
pub const KEY_H: Key = 0x68;
pub const KEY_I: Key = 0x69;
pub const KEY_J: Key = 0x6A;
pub const KEY_K: Key = 0x6B;
pub const KEY_L: Key = 0x6C;
pub const KEY_M: Key = 0x6D;
pub const KEY_N: Key = 0x6E;
pub const KEY_O: Key = 0x6F;
pub const KEY_P: Key = 0x70;
pub const KEY_Q: Key = 0x71;
pub const KEY_R: Key = 0x72;
pub const KEY_S: Key = 0x73;
pub const KEY_T: Key = 0x74;
pub const KEY_U: Key = 0x75;
pub const KEY_V: Key = 0x76;
pub const KEY_W: Key = 0x77;
pub const KEY_X: Key = 0x78;
pub const KEY_Y: Key = 0x79;
pub const KEY_Z: Key = 0x7A;
pub const KEY_DELETE: Key = 0x7F;
pub const KEY_CAPSLOCK: Key = 0x4000_0039;
pub const KEY_F1: Key = 0x4000_003A;
pub const KEY_F2: Key = 0x4000_003B;
pub const KEY_F3: Key = 0x4000_003C;
pub const KEY_F4: Key = 0x4000_003D;
pub const KEY_F5: Key = 0x4000_003E;
pub const KEY_F6: Key = 0x4000_003F;
pub const KEY_F7: Key = 0x4000_0040;
pub const KEY_F8: Key = 0x4000_0041;
pub const KEY_F9: Key = 0x4000_0042;
pub const KEY_F10: Key = 0x4000_0043;
pub const KEY_F11: Key = 0x4000_0044;
pub const KEY_F12: Key = 0x4000_0045;
pub const KEY_PRINTSCREEN: Key = 0x4000_0046;
pub const KEY_SCROLLLOCK: Key = 0x4000_0047;
pub const KEY_PAUSE: Key = 0x4000_0048;
pub const KEY_INSERT: Key = 0x4000_0049;
pub const KEY_HOME: Key = 0x4000_004A;
pub const KEY_PAGEUP: Key = 0x4000_004B;
pub const KEY_END: Key = 0x4000_004D;
pub const KEY_PAGEDOWN: Key = 0x4000_004E;
pub const KEY_RIGHT: Key = 0x4000_004F;
pub const KEY_LEFT: Key = 0x4000_0050;
pub const KEY_DOWN: Key = 0x4000_0051;
pub const KEY_UP: Key = 0x4000_0052;
pub const KEY_NUMLOCKCLEAR: Key = 0x4000_0053;
pub const KEY_KP_DIVIDE: Key = 0x4000_0054;
pub const KEY_KP_MULTIPLY: Key = 0x4000_0055;
pub const KEY_KP_MINUS: Key = 0x4000_0056;
pub const KEY_KP_PLUS: Key = 0x4000_0057;
pub const KEY_KP_ENTER: Key = 0x4000_0058;
pub const KEY_KP_1: Key = 0x4000_0059;
pub const KEY_KP_2: Key = 0x4000_005A;
pub const KEY_KP_3: Key = 0x4000_005B;
pub const KEY_KP_4: Key = 0x4000_005C;
pub const KEY_KP_5: Key = 0x4000_005D;
pub const KEY_KP_6: Key = 0x4000_005E;
pub const KEY_KP_7: Key = 0x4000_005F;
pub const KEY_KP_8: Key = 0x4000_0060;
pub const KEY_KP_9: Key = 0x4000_0061;
pub const KEY_KP_0: Key = 0x4000_0062;
pub const KEY_KP_PERIOD: Key = 0x4000_0063;
pub const KEY_APPLICATION: Key = 0x4000_0065;
pub const KEY_POWER: Key = 0x4000_0066;
pub const KEY_KP_EQUALS: Key = 0x4000_0067;
pub const KEY_F13: Key = 0x4000_0068;
pub const KEY_F14: Key = 0x4000_0069;
pub const KEY_F15: Key = 0x4000_006A;
pub const KEY_F16: Key = 0x4000_006B;
pub const KEY_F17: Key = 0x4000_006C;
pub const KEY_F18: Key = 0x4000_006D;
pub const KEY_F19: Key = 0x4000_006E;
pub const KEY_F20: Key = 0x4000_006F;
pub const KEY_F21: Key = 0x4000_0070;
pub const KEY_F22: Key = 0x4000_0071;
pub const KEY_F23: Key = 0x4000_0072;
pub const KEY_F24: Key = 0x4000_0073;
pub const KEY_EXECUTE: Key = 0x4000_0074;
pub const KEY_HELP: Key = 0x4000_0075;
pub const KEY_MENU: Key = 0x4000_0076;
pub const KEY_SELECT: Key = 0x4000_0077;
pub const KEY_STOP: Key = 0x4000_0078;
pub const KEY_AGAIN: Key = 0x4000_0079;
pub const KEY_UNDO: Key = 0x4000_007A;
pub const KEY_CUT: Key = 0x4000_007B;
pub const KEY_COPY: Key = 0x4000_007C;
pub const KEY_PASTE: Key = 0x4000_007D;
pub const KEY_FIND: Key = 0x4000_007E;
pub const KEY_MUTE: Key = 0x4000_007F;
pub const KEY_VOLUMEUP: Key = 0x4000_0080;
pub const KEY_VOLUMEDOWN: Key = 0x4000_0081;
pub const KEY_KP_COMMA: Key = 0x4000_0085;
pub const KEY_KP_EQUALSAS400: Key = 0x4000_0086;
pub const KEY_ALTERASE: Key = 0x4000_0099;
pub const KEY_SYSREQ: Key = 0x4000_009A;
pub const KEY_CANCEL: Key = 0x4000_009B;
pub const KEY_CLEAR: Key = 0x4000_009C;
pub const KEY_PRIOR: Key = 0x4000_009D;
pub const KEY_RETURN2: Key = 0x4000_009E;
pub const KEY_SEPARATOR: Key = 0x4000_009F;
pub const KEY_OUT: Key = 0x4000_00A0;
pub const KEY_OPER: Key = 0x4000_00A1;
pub const KEY_CLEARAGAIN: Key = 0x4000_00A2;
pub const KEY_CRSEL: Key = 0x4000_00A3;
pub const KEY_EXSEL: Key = 0x4000_00A4;
pub const KEY_KP_00: Key = 0x4000_00B0;
pub const KEY_KP_000: Key = 0x4000_00B1;
pub const KEY_THOUSANDSSEPARATOR: Key = 0x4000_00B2;
pub const KEY_DECIMALSEPARATOR: Key = 0x4000_00B3;
pub const KEY_CURRENCYUNIT: Key = 0x4000_00B4;
pub const KEY_CURRENCYSUBUNIT: Key = 0x4000_00B5;
pub const KEY_KP_LEFTPAREN: Key = 0x4000_00B6;
pub const KEY_KP_RIGHTPAREN: Key = 0x4000_00B7;
pub const KEY_KP_LEFTBRACE: Key = 0x4000_00B8;
pub const KEY_KP_RIGHTBRACE: Key = 0x4000_00B9;
pub const KEY_KP_TAB: Key = 0x4000_00BA;
pub const KEY_KP_BACKSPACE: Key = 0x4000_00BB;
pub const KEY_KP_A: Key = 0x4000_00BC;
pub const KEY_KP_B: Key = 0x4000_00BD;
pub const KEY_KP_C: Key = 0x4000_00BE;
pub const KEY_KP_D: Key = 0x4000_00BF;
pub const KEY_KP_E: Key = 0x4000_00C0;
pub const KEY_KP_F: Key = 0x4000_00C1;
pub const KEY_KP_XOR: Key = 0x4000_00C2;
pub const KEY_KP_POWER: Key = 0x4000_00C3;
pub const KEY_KP_PERCENT: Key = 0x4000_00C4;
pub const KEY_KP_LESS: Key = 0x4000_00C5;
pub const KEY_KP_GREATER: Key = 0x4000_00C6;
pub const KEY_KP_AMPERSAND: Key = 0x4000_00C7;
pub const KEY_KP_DBLAMPERSAND: Key = 0x4000_00C8;
pub const KEY_KP_VERTICALBAR: Key = 0x4000_00C9;
pub const KEY_KP_DBLVERTICALBAR: Key = 0x4000_00CA;
pub const KEY_KP_COLON: Key = 0x4000_00CB;
pub const KEY_KP_HASH: Key = 0x4000_00CC;
pub const KEY_KP_SPACE: Key = 0x4000_00CD;
pub const KEY_KP_AT: Key = 0x4000_00CE;
pub const KEY_KP_EXCLAM: Key = 0x4000_00CF;
pub const KEY_KP_MEMSTORE: Key = 0x4000_00D0;
pub const KEY_KP_MEMRECALL: Key = 0x4000_00D1;
pub const KEY_KP_MEMCLEAR: Key = 0x4000_00D2;
pub const KEY_KP_MEMADD: Key = 0x4000_00D3;
pub const KEY_KP_MEMSUBTRACT: Key = 0x4000_00D4;
pub const KEY_KP_MEMMULTIPLY: Key = 0x4000_00D5;
pub const KEY_KP_MEMDIVIDE: Key = 0x4000_00D6;
pub const KEY_KP_PLUSMINUS: Key = 0x4000_00D7;
pub const KEY_KP_CLEAR: Key = 0x4000_00D8;
pub const KEY_KP_CLEARENTRY: Key = 0x4000_00D9;
pub const KEY_KP_BINARY: Key = 0x4000_00DA;
pub const KEY_KP_OCTAL: Key = 0x4000_00DB;
pub const KEY_KP_DECIMAL: Key = 0x4000_00DC;
pub const KEY_KP_HEXADECIMAL: Key = 0x4000_00DD;
pub const KEY_LCTRL: Key = 0x4000_00E0;
pub const KEY_LSHIFT: Key = 0x4000_00E1;
pub const KEY_LALT: Key = 0x4000_00E2;
pub const KEY_LGUI: Key = 0x4000_00E3;
pub const KEY_RCTRL: Key = 0x4000_00E4;
pub const KEY_RSHIFT: Key = 0x4000_00E5;
pub const KEY_RALT: Key = 0x4000_00E6;
pub const KEY_RGUI: Key = 0x4000_00E7;

// Virtual keys
pub const VIRTUAL_KEY_START_INDEX: Key = 0x5000_0000;
pub const KEY_NONE: Key = VIRTUAL_KEY_START_INDEX;
pub const KEY_SHIFT: Key = VIRTUAL_KEY_START_INDEX + 1;
pub const KEY_CTRL: Key = VIRTUAL_KEY_START_INDEX + 2;
pub const KEY_ALT: Key = VIRTUAL_KEY_START_INDEX + 3;
pub const KEY_GUI: Key = VIRTUAL_KEY_START_INDEX + 4;

// Mouse
pub const MOUSE_KEY_START_INDEX: Key = 0x5000_0100;
pub const MOUSE_POS_X: Key = MOUSE_KEY_START_INDEX;
pub const MOUSE_POS_Y: Key = MOUSE_KEY_START_INDEX + 1;
pub const MOUSE_WHEEL_X: Key = MOUSE_KEY_START_INDEX + 2;
pub const MOUSE_WHEEL_Y: Key = MOUSE_KEY_START_INDEX + 3;
pub const MOUSE_BUTTON_LEFT: Key = MOUSE_KEY_START_INDEX + 4;
pub const MOUSE_BUTTON_MIDDLE: Key = MOUSE_KEY_START_INDEX + 5;
pub const MOUSE_BUTTON_RIGHT: Key = MOUSE_KEY_START_INDEX + 6;
pub const MOUSE_BUTTON_X1: Key = MOUSE_KEY_START_INDEX + 7;
pub const MOUSE_BUTTON_X2: Key = MOUSE_KEY_START_INDEX + 8;

// Gamepad
pub const GAMEPAD_KEY_START_INDEX: Key = 0x5000_0200;
pub const GAMEPAD_KEY_INDEX_INTERVAL: Key = 0x100;

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
        }
    };
}

define_gamepad_keys!(GAMEPAD1, GAMEPAD_KEY_START_INDEX);

define_gamepad_keys!(
    GAMEPAD2,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL
);

define_gamepad_keys!(
    GAMEPAD3,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 2
);

define_gamepad_keys!(
    GAMEPAD4,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 3
);
