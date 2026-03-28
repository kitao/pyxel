use crate::ffi;
use crate::helpers::*;

pub unsafe fn add_module_constants(m: ffi::py_GlobalRef) {
    // Settings
    set_const_str(m, "VERSION", pyxel::VERSION);
    set_const_str(m, "BASE_DIR", pyxel::BASE_DIR);
    set_const_str(m, "WINDOW_STATE_ENV", pyxel::WINDOW_STATE_ENV);
    set_const_str(m, "WATCH_STATE_FILE_ENV", pyxel::WATCH_STATE_FILE_ENV);
    set_const_int(
        m,
        "WATCH_RESET_EXIT_CODE",
        pyxel::WATCH_RESET_EXIT_CODE as i64,
    );

    set_const_str(m, "APP_FILE_EXTENSION", pyxel::APP_FILE_EXTENSION);
    set_const_str(m, "APP_STARTUP_SCRIPT_FILE", pyxel::APP_STARTUP_SCRIPT_FILE);
    set_const_str(m, "RESOURCE_FILE_EXTENSION", pyxel::RESOURCE_FILE_EXTENSION);
    set_const_str(m, "PALETTE_FILE_EXTENSION", pyxel::PALETTE_FILE_EXTENSION);

    set_const_int(m, "NUM_COLORS", pyxel::NUM_COLORS as i64);
    set_const_int(m, "NUM_IMAGES", pyxel::NUM_IMAGES as i64);
    set_const_int(m, "IMAGE_SIZE", pyxel::IMAGE_SIZE as i64);
    set_const_int(m, "NUM_TILEMAPS", pyxel::NUM_TILEMAPS as i64);
    set_const_int(m, "TILEMAP_SIZE", pyxel::TILEMAP_SIZE as i64);
    set_const_int(m, "TILE_SIZE", pyxel::TILE_SIZE as i64);
    // DEFAULT_COLORS - set as Python list
    {
        let n = ffi::py_name(std::ffi::CString::new("DEFAULT_COLORS").unwrap().as_ptr());
        ffi::py_newlist(ffi::py_emplacedict(m, n));
        let list = ffi::py_getdict(m, n);
        for &c in &pyxel::DEFAULT_COLORS {
            ffi::py_newint(ffi::py_list_emplace(list), c as i64);
        }
    }
    set_const_int(m, "COLOR_BLACK", pyxel::COLOR_BLACK as i64);
    set_const_int(m, "COLOR_NAVY", pyxel::COLOR_NAVY as i64);
    set_const_int(m, "COLOR_PURPLE", pyxel::COLOR_PURPLE as i64);
    set_const_int(m, "COLOR_GREEN", pyxel::COLOR_GREEN as i64);
    set_const_int(m, "COLOR_BROWN", pyxel::COLOR_BROWN as i64);
    set_const_int(m, "COLOR_DARK_BLUE", pyxel::COLOR_DARK_BLUE as i64);
    set_const_int(m, "COLOR_LIGHT_BLUE", pyxel::COLOR_LIGHT_BLUE as i64);
    set_const_int(m, "COLOR_WHITE", pyxel::COLOR_WHITE as i64);
    set_const_int(m, "COLOR_RED", pyxel::COLOR_RED as i64);
    set_const_int(m, "COLOR_ORANGE", pyxel::COLOR_ORANGE as i64);
    set_const_int(m, "COLOR_YELLOW", pyxel::COLOR_YELLOW as i64);
    set_const_int(m, "COLOR_LIME", pyxel::COLOR_LIME as i64);
    set_const_int(m, "COLOR_CYAN", pyxel::COLOR_CYAN as i64);
    set_const_int(m, "COLOR_GRAY", pyxel::COLOR_GRAY as i64);
    set_const_int(m, "COLOR_PINK", pyxel::COLOR_PINK as i64);
    set_const_int(m, "COLOR_PEACH", pyxel::COLOR_PEACH as i64);
    set_const_int(m, "FONT_WIDTH", pyxel::FONT_WIDTH as i64);
    set_const_int(m, "FONT_HEIGHT", pyxel::FONT_HEIGHT as i64);

    set_const_int(m, "NUM_CHANNELS", pyxel::NUM_CHANNELS as i64);
    set_const_int(m, "NUM_TONES", pyxel::NUM_TONES as i64);
    set_const_int(m, "NUM_SOUNDS", pyxel::NUM_SOUNDS as i64);
    set_const_int(m, "NUM_MUSICS", pyxel::NUM_MUSICS as i64);
    set_const_int(m, "TONE_TRIANGLE", pyxel::TONE_TRIANGLE as i64);
    set_const_int(m, "TONE_SQUARE", pyxel::TONE_SQUARE as i64);
    set_const_int(m, "TONE_PULSE", pyxel::TONE_PULSE as i64);
    set_const_int(m, "TONE_NOISE", pyxel::TONE_NOISE as i64);
    set_const_int(m, "EFFECT_NONE", pyxel::EFFECT_NONE as i64);
    set_const_int(m, "EFFECT_SLIDE", pyxel::EFFECT_SLIDE as i64);
    set_const_int(m, "EFFECT_VIBRATO", pyxel::EFFECT_VIBRATO as i64);
    set_const_int(m, "EFFECT_FADEOUT", pyxel::EFFECT_FADEOUT as i64);
    set_const_int(m, "EFFECT_HALF_FADEOUT", pyxel::EFFECT_HALF_FADEOUT as i64);
    set_const_int(
        m,
        "EFFECT_QUARTER_FADEOUT",
        pyxel::EFFECT_QUARTER_FADEOUT as i64,
    );

    // Key
    set_const_int(m, "KEY_UNKNOWN", pyxel::KEY_UNKNOWN as i64);
    set_const_int(m, "KEY_BACKSPACE", pyxel::KEY_BACKSPACE as i64);
    set_const_int(m, "KEY_TAB", pyxel::KEY_TAB as i64);
    set_const_int(m, "KEY_RETURN", pyxel::KEY_RETURN as i64);
    set_const_int(m, "KEY_ESCAPE", pyxel::KEY_ESCAPE as i64);
    set_const_int(m, "KEY_SPACE", pyxel::KEY_SPACE as i64);
    set_const_int(m, "KEY_EXCLAIM", pyxel::KEY_EXCLAIM as i64);
    set_const_int(m, "KEY_QUOTEDBL", pyxel::KEY_QUOTEDBL as i64);
    set_const_int(m, "KEY_HASH", pyxel::KEY_HASH as i64);
    set_const_int(m, "KEY_DOLLAR", pyxel::KEY_DOLLAR as i64);
    set_const_int(m, "KEY_PERCENT", pyxel::KEY_PERCENT as i64);
    set_const_int(m, "KEY_AMPERSAND", pyxel::KEY_AMPERSAND as i64);
    set_const_int(m, "KEY_QUOTE", pyxel::KEY_QUOTE as i64);
    set_const_int(m, "KEY_LEFTPAREN", pyxel::KEY_LEFTPAREN as i64);
    set_const_int(m, "KEY_RIGHTPAREN", pyxel::KEY_RIGHTPAREN as i64);
    set_const_int(m, "KEY_ASTERISK", pyxel::KEY_ASTERISK as i64);
    set_const_int(m, "KEY_PLUS", pyxel::KEY_PLUS as i64);
    set_const_int(m, "KEY_COMMA", pyxel::KEY_COMMA as i64);
    set_const_int(m, "KEY_MINUS", pyxel::KEY_MINUS as i64);
    set_const_int(m, "KEY_PERIOD", pyxel::KEY_PERIOD as i64);
    set_const_int(m, "KEY_SLASH", pyxel::KEY_SLASH as i64);
    set_const_int(m, "KEY_0", pyxel::KEY_0 as i64);
    set_const_int(m, "KEY_1", pyxel::KEY_1 as i64);
    set_const_int(m, "KEY_2", pyxel::KEY_2 as i64);
    set_const_int(m, "KEY_3", pyxel::KEY_3 as i64);
    set_const_int(m, "KEY_4", pyxel::KEY_4 as i64);
    set_const_int(m, "KEY_5", pyxel::KEY_5 as i64);
    set_const_int(m, "KEY_6", pyxel::KEY_6 as i64);
    set_const_int(m, "KEY_7", pyxel::KEY_7 as i64);
    set_const_int(m, "KEY_8", pyxel::KEY_8 as i64);
    set_const_int(m, "KEY_9", pyxel::KEY_9 as i64);
    set_const_int(m, "KEY_COLON", pyxel::KEY_COLON as i64);
    set_const_int(m, "KEY_SEMICOLON", pyxel::KEY_SEMICOLON as i64);
    set_const_int(m, "KEY_LESS", pyxel::KEY_LESS as i64);
    set_const_int(m, "KEY_EQUALS", pyxel::KEY_EQUALS as i64);
    set_const_int(m, "KEY_GREATER", pyxel::KEY_GREATER as i64);
    set_const_int(m, "KEY_QUESTION", pyxel::KEY_QUESTION as i64);
    set_const_int(m, "KEY_AT", pyxel::KEY_AT as i64);
    set_const_int(m, "KEY_LEFTBRACKET", pyxel::KEY_LEFTBRACKET as i64);
    set_const_int(m, "KEY_BACKSLASH", pyxel::KEY_BACKSLASH as i64);
    set_const_int(m, "KEY_RIGHTBRACKET", pyxel::KEY_RIGHTBRACKET as i64);
    set_const_int(m, "KEY_CARET", pyxel::KEY_CARET as i64);
    set_const_int(m, "KEY_UNDERSCORE", pyxel::KEY_UNDERSCORE as i64);
    set_const_int(m, "KEY_BACKQUOTE", pyxel::KEY_BACKQUOTE as i64);
    set_const_int(m, "KEY_A", pyxel::KEY_A as i64);
    set_const_int(m, "KEY_B", pyxel::KEY_B as i64);
    set_const_int(m, "KEY_C", pyxel::KEY_C as i64);
    set_const_int(m, "KEY_D", pyxel::KEY_D as i64);
    set_const_int(m, "KEY_E", pyxel::KEY_E as i64);
    set_const_int(m, "KEY_F", pyxel::KEY_F as i64);
    set_const_int(m, "KEY_G", pyxel::KEY_G as i64);
    set_const_int(m, "KEY_H", pyxel::KEY_H as i64);
    set_const_int(m, "KEY_I", pyxel::KEY_I as i64);
    set_const_int(m, "KEY_J", pyxel::KEY_J as i64);
    set_const_int(m, "KEY_K", pyxel::KEY_K as i64);
    set_const_int(m, "KEY_L", pyxel::KEY_L as i64);
    set_const_int(m, "KEY_M", pyxel::KEY_M as i64);
    set_const_int(m, "KEY_N", pyxel::KEY_N as i64);
    set_const_int(m, "KEY_O", pyxel::KEY_O as i64);
    set_const_int(m, "KEY_P", pyxel::KEY_P as i64);
    set_const_int(m, "KEY_Q", pyxel::KEY_Q as i64);
    set_const_int(m, "KEY_R", pyxel::KEY_R as i64);
    set_const_int(m, "KEY_S", pyxel::KEY_S as i64);
    set_const_int(m, "KEY_T", pyxel::KEY_T as i64);
    set_const_int(m, "KEY_U", pyxel::KEY_U as i64);
    set_const_int(m, "KEY_V", pyxel::KEY_V as i64);
    set_const_int(m, "KEY_W", pyxel::KEY_W as i64);
    set_const_int(m, "KEY_X", pyxel::KEY_X as i64);
    set_const_int(m, "KEY_Y", pyxel::KEY_Y as i64);
    set_const_int(m, "KEY_Z", pyxel::KEY_Z as i64);
    set_const_int(m, "KEY_DELETE", pyxel::KEY_DELETE as i64);
    set_const_int(m, "KEY_CAPSLOCK", pyxel::KEY_CAPSLOCK as i64);
    set_const_int(m, "KEY_F1", pyxel::KEY_F1 as i64);
    set_const_int(m, "KEY_F2", pyxel::KEY_F2 as i64);
    set_const_int(m, "KEY_F3", pyxel::KEY_F3 as i64);
    set_const_int(m, "KEY_F4", pyxel::KEY_F4 as i64);
    set_const_int(m, "KEY_F5", pyxel::KEY_F5 as i64);
    set_const_int(m, "KEY_F6", pyxel::KEY_F6 as i64);
    set_const_int(m, "KEY_F7", pyxel::KEY_F7 as i64);
    set_const_int(m, "KEY_F8", pyxel::KEY_F8 as i64);
    set_const_int(m, "KEY_F9", pyxel::KEY_F9 as i64);
    set_const_int(m, "KEY_F10", pyxel::KEY_F10 as i64);
    set_const_int(m, "KEY_F11", pyxel::KEY_F11 as i64);
    set_const_int(m, "KEY_F12", pyxel::KEY_F12 as i64);
    set_const_int(m, "KEY_PRINTSCREEN", pyxel::KEY_PRINTSCREEN as i64);
    set_const_int(m, "KEY_SCROLLLOCK", pyxel::KEY_SCROLLLOCK as i64);
    set_const_int(m, "KEY_PAUSE", pyxel::KEY_PAUSE as i64);
    set_const_int(m, "KEY_INSERT", pyxel::KEY_INSERT as i64);
    set_const_int(m, "KEY_HOME", pyxel::KEY_HOME as i64);
    set_const_int(m, "KEY_PAGEUP", pyxel::KEY_PAGEUP as i64);
    set_const_int(m, "KEY_END", pyxel::KEY_END as i64);
    set_const_int(m, "KEY_PAGEDOWN", pyxel::KEY_PAGEDOWN as i64);
    set_const_int(m, "KEY_RIGHT", pyxel::KEY_RIGHT as i64);
    set_const_int(m, "KEY_LEFT", pyxel::KEY_LEFT as i64);
    set_const_int(m, "KEY_DOWN", pyxel::KEY_DOWN as i64);
    set_const_int(m, "KEY_UP", pyxel::KEY_UP as i64);
    set_const_int(m, "KEY_NUMLOCKCLEAR", pyxel::KEY_NUMLOCKCLEAR as i64);
    set_const_int(m, "KEY_KP_DIVIDE", pyxel::KEY_KP_DIVIDE as i64);
    set_const_int(m, "KEY_KP_MULTIPLY", pyxel::KEY_KP_MULTIPLY as i64);
    set_const_int(m, "KEY_KP_MINUS", pyxel::KEY_KP_MINUS as i64);
    set_const_int(m, "KEY_KP_PLUS", pyxel::KEY_KP_PLUS as i64);
    set_const_int(m, "KEY_KP_ENTER", pyxel::KEY_KP_ENTER as i64);
    set_const_int(m, "KEY_KP_1", pyxel::KEY_KP_1 as i64);
    set_const_int(m, "KEY_KP_2", pyxel::KEY_KP_2 as i64);
    set_const_int(m, "KEY_KP_3", pyxel::KEY_KP_3 as i64);
    set_const_int(m, "KEY_KP_4", pyxel::KEY_KP_4 as i64);
    set_const_int(m, "KEY_KP_5", pyxel::KEY_KP_5 as i64);
    set_const_int(m, "KEY_KP_6", pyxel::KEY_KP_6 as i64);
    set_const_int(m, "KEY_KP_7", pyxel::KEY_KP_7 as i64);
    set_const_int(m, "KEY_KP_8", pyxel::KEY_KP_8 as i64);
    set_const_int(m, "KEY_KP_9", pyxel::KEY_KP_9 as i64);
    set_const_int(m, "KEY_KP_0", pyxel::KEY_KP_0 as i64);
    set_const_int(m, "KEY_KP_PERIOD", pyxel::KEY_KP_PERIOD as i64);
    set_const_int(m, "KEY_APPLICATION", pyxel::KEY_APPLICATION as i64);
    set_const_int(m, "KEY_POWER", pyxel::KEY_POWER as i64);
    set_const_int(m, "KEY_KP_EQUALS", pyxel::KEY_KP_EQUALS as i64);
    set_const_int(m, "KEY_F13", pyxel::KEY_F13 as i64);
    set_const_int(m, "KEY_F14", pyxel::KEY_F14 as i64);
    set_const_int(m, "KEY_F15", pyxel::KEY_F15 as i64);
    set_const_int(m, "KEY_F16", pyxel::KEY_F16 as i64);
    set_const_int(m, "KEY_F17", pyxel::KEY_F17 as i64);
    set_const_int(m, "KEY_F18", pyxel::KEY_F18 as i64);
    set_const_int(m, "KEY_F19", pyxel::KEY_F19 as i64);
    set_const_int(m, "KEY_F20", pyxel::KEY_F20 as i64);
    set_const_int(m, "KEY_F21", pyxel::KEY_F21 as i64);
    set_const_int(m, "KEY_F22", pyxel::KEY_F22 as i64);
    set_const_int(m, "KEY_F23", pyxel::KEY_F23 as i64);
    set_const_int(m, "KEY_F24", pyxel::KEY_F24 as i64);
    set_const_int(m, "KEY_EXECUTE", pyxel::KEY_EXECUTE as i64);
    set_const_int(m, "KEY_HELP", pyxel::KEY_HELP as i64);
    set_const_int(m, "KEY_MENU", pyxel::KEY_MENU as i64);
    set_const_int(m, "KEY_SELECT", pyxel::KEY_SELECT as i64);
    set_const_int(m, "KEY_STOP", pyxel::KEY_STOP as i64);
    set_const_int(m, "KEY_AGAIN", pyxel::KEY_AGAIN as i64);
    set_const_int(m, "KEY_UNDO", pyxel::KEY_UNDO as i64);
    set_const_int(m, "KEY_CUT", pyxel::KEY_CUT as i64);
    set_const_int(m, "KEY_COPY", pyxel::KEY_COPY as i64);
    set_const_int(m, "KEY_PASTE", pyxel::KEY_PASTE as i64);
    set_const_int(m, "KEY_FIND", pyxel::KEY_FIND as i64);
    set_const_int(m, "KEY_MUTE", pyxel::KEY_MUTE as i64);
    set_const_int(m, "KEY_VOLUMEUP", pyxel::KEY_VOLUMEUP as i64);
    set_const_int(m, "KEY_VOLUMEDOWN", pyxel::KEY_VOLUMEDOWN as i64);
    set_const_int(m, "KEY_KP_COMMA", pyxel::KEY_KP_COMMA as i64);
    set_const_int(m, "KEY_KP_EQUALSAS400", pyxel::KEY_KP_EQUALSAS400 as i64);
    set_const_int(m, "KEY_ALTERASE", pyxel::KEY_ALTERASE as i64);
    set_const_int(m, "KEY_SYSREQ", pyxel::KEY_SYSREQ as i64);
    set_const_int(m, "KEY_CANCEL", pyxel::KEY_CANCEL as i64);
    set_const_int(m, "KEY_CLEAR", pyxel::KEY_CLEAR as i64);
    set_const_int(m, "KEY_PRIOR", pyxel::KEY_PRIOR as i64);
    set_const_int(m, "KEY_RETURN2", pyxel::KEY_RETURN2 as i64);
    set_const_int(m, "KEY_SEPARATOR", pyxel::KEY_SEPARATOR as i64);
    set_const_int(m, "KEY_OUT", pyxel::KEY_OUT as i64);
    set_const_int(m, "KEY_OPER", pyxel::KEY_OPER as i64);
    set_const_int(m, "KEY_CLEARAGAIN", pyxel::KEY_CLEARAGAIN as i64);
    set_const_int(m, "KEY_CRSEL", pyxel::KEY_CRSEL as i64);
    set_const_int(m, "KEY_EXSEL", pyxel::KEY_EXSEL as i64);
    set_const_int(m, "KEY_KP_00", pyxel::KEY_KP_00 as i64);
    set_const_int(m, "KEY_KP_000", pyxel::KEY_KP_000 as i64);
    set_const_int(
        m,
        "KEY_THOUSANDSSEPARATOR",
        pyxel::KEY_THOUSANDSSEPARATOR as i64,
    );
    set_const_int(
        m,
        "KEY_DECIMALSEPARATOR",
        pyxel::KEY_DECIMALSEPARATOR as i64,
    );
    set_const_int(m, "KEY_CURRENCYUNIT", pyxel::KEY_CURRENCYUNIT as i64);
    set_const_int(m, "KEY_CURRENCYSUBUNIT", pyxel::KEY_CURRENCYSUBUNIT as i64);
    set_const_int(m, "KEY_KP_LEFTPAREN", pyxel::KEY_KP_LEFTPAREN as i64);
    set_const_int(m, "KEY_KP_RIGHTPAREN", pyxel::KEY_KP_RIGHTPAREN as i64);
    set_const_int(m, "KEY_KP_LEFTBRACE", pyxel::KEY_KP_LEFTBRACE as i64);
    set_const_int(m, "KEY_KP_RIGHTBRACE", pyxel::KEY_KP_RIGHTBRACE as i64);
    set_const_int(m, "KEY_KP_TAB", pyxel::KEY_KP_TAB as i64);
    set_const_int(m, "KEY_KP_BACKSPACE", pyxel::KEY_KP_BACKSPACE as i64);
    set_const_int(m, "KEY_KP_A", pyxel::KEY_KP_A as i64);
    set_const_int(m, "KEY_KP_B", pyxel::KEY_KP_B as i64);
    set_const_int(m, "KEY_KP_C", pyxel::KEY_KP_C as i64);
    set_const_int(m, "KEY_KP_D", pyxel::KEY_KP_D as i64);
    set_const_int(m, "KEY_KP_E", pyxel::KEY_KP_E as i64);
    set_const_int(m, "KEY_KP_F", pyxel::KEY_KP_F as i64);
    set_const_int(m, "KEY_KP_XOR", pyxel::KEY_KP_XOR as i64);
    set_const_int(m, "KEY_KP_POWER", pyxel::KEY_KP_POWER as i64);
    set_const_int(m, "KEY_KP_PERCENT", pyxel::KEY_KP_PERCENT as i64);
    set_const_int(m, "KEY_KP_LESS", pyxel::KEY_KP_LESS as i64);
    set_const_int(m, "KEY_KP_GREATER", pyxel::KEY_KP_GREATER as i64);
    set_const_int(m, "KEY_KP_AMPERSAND", pyxel::KEY_KP_AMPERSAND as i64);
    set_const_int(m, "KEY_KP_DBLAMPERSAND", pyxel::KEY_KP_DBLAMPERSAND as i64);
    set_const_int(m, "KEY_KP_VERTICALBAR", pyxel::KEY_KP_VERTICALBAR as i64);
    set_const_int(
        m,
        "KEY_KP_DBLVERTICALBAR",
        pyxel::KEY_KP_DBLVERTICALBAR as i64,
    );
    set_const_int(m, "KEY_KP_COLON", pyxel::KEY_KP_COLON as i64);
    set_const_int(m, "KEY_KP_HASH", pyxel::KEY_KP_HASH as i64);
    set_const_int(m, "KEY_KP_SPACE", pyxel::KEY_KP_SPACE as i64);
    set_const_int(m, "KEY_KP_AT", pyxel::KEY_KP_AT as i64);
    set_const_int(m, "KEY_KP_EXCLAM", pyxel::KEY_KP_EXCLAM as i64);
    set_const_int(m, "KEY_KP_MEMSTORE", pyxel::KEY_KP_MEMSTORE as i64);
    set_const_int(m, "KEY_KP_MEMRECALL", pyxel::KEY_KP_MEMRECALL as i64);
    set_const_int(m, "KEY_KP_MEMCLEAR", pyxel::KEY_KP_MEMCLEAR as i64);
    set_const_int(m, "KEY_KP_MEMADD", pyxel::KEY_KP_MEMADD as i64);
    set_const_int(m, "KEY_KP_MEMSUBTRACT", pyxel::KEY_KP_MEMSUBTRACT as i64);
    set_const_int(m, "KEY_KP_MEMMULTIPLY", pyxel::KEY_KP_MEMMULTIPLY as i64);
    set_const_int(m, "KEY_KP_MEMDIVIDE", pyxel::KEY_KP_MEMDIVIDE as i64);
    set_const_int(m, "KEY_KP_PLUSMINUS", pyxel::KEY_KP_PLUSMINUS as i64);
    set_const_int(m, "KEY_KP_CLEAR", pyxel::KEY_KP_CLEAR as i64);
    set_const_int(m, "KEY_KP_CLEARENTRY", pyxel::KEY_KP_CLEARENTRY as i64);
    set_const_int(m, "KEY_KP_BINARY", pyxel::KEY_KP_BINARY as i64);
    set_const_int(m, "KEY_KP_OCTAL", pyxel::KEY_KP_OCTAL as i64);
    set_const_int(m, "KEY_KP_DECIMAL", pyxel::KEY_KP_DECIMAL as i64);
    set_const_int(m, "KEY_KP_HEXADECIMAL", pyxel::KEY_KP_HEXADECIMAL as i64);
    set_const_int(m, "KEY_LCTRL", pyxel::KEY_LCTRL as i64);
    set_const_int(m, "KEY_LSHIFT", pyxel::KEY_LSHIFT as i64);
    set_const_int(m, "KEY_LALT", pyxel::KEY_LALT as i64);
    set_const_int(m, "KEY_LGUI", pyxel::KEY_LGUI as i64);
    set_const_int(m, "KEY_RCTRL", pyxel::KEY_RCTRL as i64);
    set_const_int(m, "KEY_RSHIFT", pyxel::KEY_RSHIFT as i64);
    set_const_int(m, "KEY_RALT", pyxel::KEY_RALT as i64);
    set_const_int(m, "KEY_RGUI", pyxel::KEY_RGUI as i64);

    set_const_int(m, "KEY_NONE", pyxel::KEY_NONE as i64);
    set_const_int(m, "KEY_SHIFT", pyxel::KEY_SHIFT as i64);
    set_const_int(m, "KEY_CTRL", pyxel::KEY_CTRL as i64);
    set_const_int(m, "KEY_ALT", pyxel::KEY_ALT as i64);
    set_const_int(m, "KEY_GUI", pyxel::KEY_GUI as i64);

    set_const_int(m, "MOUSE_POS_X", pyxel::MOUSE_POS_X as i64);
    set_const_int(m, "MOUSE_POS_Y", pyxel::MOUSE_POS_Y as i64);
    set_const_int(m, "MOUSE_WHEEL_X", pyxel::MOUSE_WHEEL_X as i64);
    set_const_int(m, "MOUSE_WHEEL_Y", pyxel::MOUSE_WHEEL_Y as i64);
    set_const_int(m, "MOUSE_BUTTON_LEFT", pyxel::MOUSE_BUTTON_LEFT as i64);
    set_const_int(m, "MOUSE_BUTTON_MIDDLE", pyxel::MOUSE_BUTTON_MIDDLE as i64);
    set_const_int(m, "MOUSE_BUTTON_RIGHT", pyxel::MOUSE_BUTTON_RIGHT as i64);
    set_const_int(m, "MOUSE_BUTTON_X1", pyxel::MOUSE_BUTTON_X1 as i64);
    set_const_int(m, "MOUSE_BUTTON_X2", pyxel::MOUSE_BUTTON_X2 as i64);

    set_const_int(m, "GAMEPAD1_AXIS_LEFTX", pyxel::GAMEPAD1_AXIS_LEFTX as i64);
    set_const_int(m, "GAMEPAD1_AXIS_LEFTY", pyxel::GAMEPAD1_AXIS_LEFTY as i64);
    set_const_int(
        m,
        "GAMEPAD1_AXIS_RIGHTX",
        pyxel::GAMEPAD1_AXIS_RIGHTX as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_AXIS_RIGHTY",
        pyxel::GAMEPAD1_AXIS_RIGHTY as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_AXIS_TRIGGERLEFT",
        pyxel::GAMEPAD1_AXIS_TRIGGERLEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_AXIS_TRIGGERRIGHT",
        pyxel::GAMEPAD1_AXIS_TRIGGERRIGHT as i64,
    );
    set_const_int(m, "GAMEPAD1_BUTTON_A", pyxel::GAMEPAD1_BUTTON_A as i64);
    set_const_int(m, "GAMEPAD1_BUTTON_B", pyxel::GAMEPAD1_BUTTON_B as i64);
    set_const_int(m, "GAMEPAD1_BUTTON_X", pyxel::GAMEPAD1_BUTTON_X as i64);
    set_const_int(m, "GAMEPAD1_BUTTON_Y", pyxel::GAMEPAD1_BUTTON_Y as i64);
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_BACK",
        pyxel::GAMEPAD1_BUTTON_BACK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_GUIDE",
        pyxel::GAMEPAD1_BUTTON_GUIDE as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_START",
        pyxel::GAMEPAD1_BUTTON_START as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_LEFTSTICK",
        pyxel::GAMEPAD1_BUTTON_LEFTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_RIGHTSTICK",
        pyxel::GAMEPAD1_BUTTON_RIGHTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_LEFTSHOULDER",
        pyxel::GAMEPAD1_BUTTON_LEFTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_RIGHTSHOULDER",
        pyxel::GAMEPAD1_BUTTON_RIGHTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_DPAD_UP",
        pyxel::GAMEPAD1_BUTTON_DPAD_UP as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_DPAD_DOWN",
        pyxel::GAMEPAD1_BUTTON_DPAD_DOWN as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_DPAD_LEFT",
        pyxel::GAMEPAD1_BUTTON_DPAD_LEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD1_BUTTON_DPAD_RIGHT",
        pyxel::GAMEPAD1_BUTTON_DPAD_RIGHT as i64,
    );

    set_const_int(m, "GAMEPAD2_AXIS_LEFTX", pyxel::GAMEPAD2_AXIS_LEFTX as i64);
    set_const_int(m, "GAMEPAD2_AXIS_LEFTY", pyxel::GAMEPAD2_AXIS_LEFTY as i64);
    set_const_int(
        m,
        "GAMEPAD2_AXIS_RIGHTX",
        pyxel::GAMEPAD2_AXIS_RIGHTX as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_AXIS_RIGHTY",
        pyxel::GAMEPAD2_AXIS_RIGHTY as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_AXIS_TRIGGERLEFT",
        pyxel::GAMEPAD2_AXIS_TRIGGERLEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_AXIS_TRIGGERRIGHT",
        pyxel::GAMEPAD2_AXIS_TRIGGERRIGHT as i64,
    );
    set_const_int(m, "GAMEPAD2_BUTTON_A", pyxel::GAMEPAD2_BUTTON_A as i64);
    set_const_int(m, "GAMEPAD2_BUTTON_B", pyxel::GAMEPAD2_BUTTON_B as i64);
    set_const_int(m, "GAMEPAD2_BUTTON_X", pyxel::GAMEPAD2_BUTTON_X as i64);
    set_const_int(m, "GAMEPAD2_BUTTON_Y", pyxel::GAMEPAD2_BUTTON_Y as i64);
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_BACK",
        pyxel::GAMEPAD2_BUTTON_BACK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_GUIDE",
        pyxel::GAMEPAD2_BUTTON_GUIDE as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_START",
        pyxel::GAMEPAD2_BUTTON_START as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_LEFTSTICK",
        pyxel::GAMEPAD2_BUTTON_LEFTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_RIGHTSTICK",
        pyxel::GAMEPAD2_BUTTON_RIGHTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_LEFTSHOULDER",
        pyxel::GAMEPAD2_BUTTON_LEFTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_RIGHTSHOULDER",
        pyxel::GAMEPAD2_BUTTON_RIGHTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_DPAD_UP",
        pyxel::GAMEPAD2_BUTTON_DPAD_UP as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_DPAD_DOWN",
        pyxel::GAMEPAD2_BUTTON_DPAD_DOWN as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_DPAD_LEFT",
        pyxel::GAMEPAD2_BUTTON_DPAD_LEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD2_BUTTON_DPAD_RIGHT",
        pyxel::GAMEPAD2_BUTTON_DPAD_RIGHT as i64,
    );

    set_const_int(m, "GAMEPAD3_AXIS_LEFTX", pyxel::GAMEPAD3_AXIS_LEFTX as i64);
    set_const_int(m, "GAMEPAD3_AXIS_LEFTY", pyxel::GAMEPAD3_AXIS_LEFTY as i64);
    set_const_int(
        m,
        "GAMEPAD3_AXIS_RIGHTX",
        pyxel::GAMEPAD3_AXIS_RIGHTX as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_AXIS_RIGHTY",
        pyxel::GAMEPAD3_AXIS_RIGHTY as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_AXIS_TRIGGERLEFT",
        pyxel::GAMEPAD3_AXIS_TRIGGERLEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_AXIS_TRIGGERRIGHT",
        pyxel::GAMEPAD3_AXIS_TRIGGERRIGHT as i64,
    );
    set_const_int(m, "GAMEPAD3_BUTTON_A", pyxel::GAMEPAD3_BUTTON_A as i64);
    set_const_int(m, "GAMEPAD3_BUTTON_B", pyxel::GAMEPAD3_BUTTON_B as i64);
    set_const_int(m, "GAMEPAD3_BUTTON_X", pyxel::GAMEPAD3_BUTTON_X as i64);
    set_const_int(m, "GAMEPAD3_BUTTON_Y", pyxel::GAMEPAD3_BUTTON_Y as i64);
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_BACK",
        pyxel::GAMEPAD3_BUTTON_BACK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_GUIDE",
        pyxel::GAMEPAD3_BUTTON_GUIDE as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_START",
        pyxel::GAMEPAD3_BUTTON_START as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_LEFTSTICK",
        pyxel::GAMEPAD3_BUTTON_LEFTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_RIGHTSTICK",
        pyxel::GAMEPAD3_BUTTON_RIGHTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_LEFTSHOULDER",
        pyxel::GAMEPAD3_BUTTON_LEFTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_RIGHTSHOULDER",
        pyxel::GAMEPAD3_BUTTON_RIGHTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_DPAD_UP",
        pyxel::GAMEPAD3_BUTTON_DPAD_UP as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_DPAD_DOWN",
        pyxel::GAMEPAD3_BUTTON_DPAD_DOWN as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_DPAD_LEFT",
        pyxel::GAMEPAD3_BUTTON_DPAD_LEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD3_BUTTON_DPAD_RIGHT",
        pyxel::GAMEPAD3_BUTTON_DPAD_RIGHT as i64,
    );

    set_const_int(m, "GAMEPAD4_AXIS_LEFTX", pyxel::GAMEPAD4_AXIS_LEFTX as i64);
    set_const_int(m, "GAMEPAD4_AXIS_LEFTY", pyxel::GAMEPAD4_AXIS_LEFTY as i64);
    set_const_int(
        m,
        "GAMEPAD4_AXIS_RIGHTX",
        pyxel::GAMEPAD4_AXIS_RIGHTX as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_AXIS_RIGHTY",
        pyxel::GAMEPAD4_AXIS_RIGHTY as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_AXIS_TRIGGERLEFT",
        pyxel::GAMEPAD4_AXIS_TRIGGERLEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_AXIS_TRIGGERRIGHT",
        pyxel::GAMEPAD4_AXIS_TRIGGERRIGHT as i64,
    );
    set_const_int(m, "GAMEPAD4_BUTTON_A", pyxel::GAMEPAD4_BUTTON_A as i64);
    set_const_int(m, "GAMEPAD4_BUTTON_B", pyxel::GAMEPAD4_BUTTON_B as i64);
    set_const_int(m, "GAMEPAD4_BUTTON_X", pyxel::GAMEPAD4_BUTTON_X as i64);
    set_const_int(m, "GAMEPAD4_BUTTON_Y", pyxel::GAMEPAD4_BUTTON_Y as i64);
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_BACK",
        pyxel::GAMEPAD4_BUTTON_BACK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_GUIDE",
        pyxel::GAMEPAD4_BUTTON_GUIDE as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_START",
        pyxel::GAMEPAD4_BUTTON_START as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_LEFTSTICK",
        pyxel::GAMEPAD4_BUTTON_LEFTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_RIGHTSTICK",
        pyxel::GAMEPAD4_BUTTON_RIGHTSTICK as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_LEFTSHOULDER",
        pyxel::GAMEPAD4_BUTTON_LEFTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_RIGHTSHOULDER",
        pyxel::GAMEPAD4_BUTTON_RIGHTSHOULDER as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_DPAD_UP",
        pyxel::GAMEPAD4_BUTTON_DPAD_UP as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_DPAD_DOWN",
        pyxel::GAMEPAD4_BUTTON_DPAD_DOWN as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_DPAD_LEFT",
        pyxel::GAMEPAD4_BUTTON_DPAD_LEFT as i64,
    );
    set_const_int(
        m,
        "GAMEPAD4_BUTTON_DPAD_RIGHT",
        pyxel::GAMEPAD4_BUTTON_DPAD_RIGHT as i64,
    );
}
