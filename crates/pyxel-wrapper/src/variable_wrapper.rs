use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use pyxel::{Color, Rgb8};

use crate::image_wrapper::wrap_pyxel_image;
use crate::instance;

macro_rules! add_constant {
    ($m: ident, $name: ident) => {
        $m.add(stringify!($name), pyxel::$name)
    };
}

#[pyclass]
struct Colors;

#[pyproto]
impl PySequenceProtocol for Colors {
    fn __len__(&self) -> usize {
        instance().colors.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Rgb8> {
        Ok(instance().colors[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, value: Rgb8) {
        instance().colors[idx as usize] = value;
    }
}

#[pyclass]
struct Palette;

#[pyproto]
impl PySequenceProtocol for Palette {
    fn __len__(&self) -> usize {
        instance().palette.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Color> {
        Ok(instance().palette[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, value: Color) {
        instance().palette[idx as usize] = value;
    }
}

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        //
        // System
        //
        "width" => instance().width().to_object(py),
        "height" => instance().height().to_object(py),
        "frame_count" => instance().frame_count().to_object(py),

        //
        // Input
        //
        "mouse_x" => instance().mouse_x().to_object(py),
        "mouse_y" => instance().mouse_y().to_object(py),
        "mouse_wheel" => instance().mouse_wheel().to_object(py),
        "text_input" => instance().text_input().to_object(py),
        "drop_files" => instance().drop_files().to_object(py),

        //
        // Graphics
        //
        "colors" => Py::new(py, Colors)?.into_py(py),
        "palette" => Py::new(py, Palette)?.into_py(py),
        "screen" => wrap_pyxel_image(instance().screen.clone()).into_py(py),
        "cursor" => wrap_pyxel_image(instance().cursor.clone()).into_py(py),
        "font" => wrap_pyxel_image(instance().font.clone()).into_py(py),

        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{}'",
                name
            )))
        }
    };

    Ok(value)
}

pub fn add_module_variables(m: &PyModule) -> PyResult<()> {
    m.add_class::<Colors>()?;
    m.add_class::<Palette>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;

    // settings
    add_constant!(m, PYXEL_VERSION)?;

    add_constant!(m, COLOR_COUNT)?;
    add_constant!(m, IMAGE_COUNT)?;
    add_constant!(m, IMAGE_SIZE)?;
    add_constant!(m, TILEMAP_COUNT)?;
    add_constant!(m, TILEMAP_SIZE)?;
    add_constant!(m, TILE_SIZE)?;

    add_constant!(m, COLOR_BLACK)?;
    add_constant!(m, COLOR_NAVY)?;
    add_constant!(m, COLOR_PURPLE)?;
    add_constant!(m, COLOR_GREEN)?;
    add_constant!(m, COLOR_BROWN)?;
    add_constant!(m, COLOR_DARK_BLUE)?;
    add_constant!(m, COLOR_LIGHT_BLUE)?;
    add_constant!(m, COLOR_WHITE)?;
    add_constant!(m, COLOR_RED)?;
    add_constant!(m, COLOR_ORANGE)?;
    add_constant!(m, COLOR_YELLOW)?;
    add_constant!(m, COLOR_LIME)?;
    add_constant!(m, COLOR_CYAN)?;
    add_constant!(m, COLOR_GRAY)?;
    add_constant!(m, COLOR_PINK)?;
    add_constant!(m, COLOR_PEACH)?;

    add_constant!(m, FONT_WIDTH)?;
    add_constant!(m, FONT_HEIGHT)?;

    add_constant!(m, CHANNEL_COUNT)?;
    add_constant!(m, SOUND_COUNT)?;
    add_constant!(m, MUSIC_COUNT)?;

    add_constant!(m, TONE_TRIANGLE)?;
    add_constant!(m, TONE_SQUARE)?;
    add_constant!(m, TONE_PULSE)?;
    add_constant!(m, TONE_NOISE)?;

    add_constant!(m, EFFECT_NONE)?;
    add_constant!(m, EFFECT_SLIDE)?;
    add_constant!(m, EFFECT_VIBRATO)?;
    add_constant!(m, EFFECT_FADEOUT)?;

    // keys
    add_constant!(m, KEY_NONE)?;

    add_constant!(m, KEY_A)?;
    add_constant!(m, KEY_B)?;
    add_constant!(m, KEY_C)?;
    add_constant!(m, KEY_D)?;
    add_constant!(m, KEY_E)?;
    add_constant!(m, KEY_F)?;
    add_constant!(m, KEY_G)?;
    add_constant!(m, KEY_H)?;
    add_constant!(m, KEY_I)?;
    add_constant!(m, KEY_J)?;
    add_constant!(m, KEY_K)?;
    add_constant!(m, KEY_L)?;
    add_constant!(m, KEY_M)?;
    add_constant!(m, KEY_N)?;
    add_constant!(m, KEY_O)?;
    add_constant!(m, KEY_P)?;
    add_constant!(m, KEY_Q)?;
    add_constant!(m, KEY_R)?;
    add_constant!(m, KEY_S)?;
    add_constant!(m, KEY_T)?;
    add_constant!(m, KEY_U)?;
    add_constant!(m, KEY_V)?;
    add_constant!(m, KEY_W)?;
    add_constant!(m, KEY_X)?;
    add_constant!(m, KEY_Y)?;
    add_constant!(m, KEY_Z)?;
    add_constant!(m, KEY_1)?;
    add_constant!(m, KEY_2)?;
    add_constant!(m, KEY_3)?;
    add_constant!(m, KEY_4)?;
    add_constant!(m, KEY_5)?;
    add_constant!(m, KEY_6)?;
    add_constant!(m, KEY_7)?;
    add_constant!(m, KEY_8)?;
    add_constant!(m, KEY_9)?;
    add_constant!(m, KEY_0)?;
    add_constant!(m, KEY_RETURN)?;
    add_constant!(m, KEY_ESCAPE)?;
    add_constant!(m, KEY_BACKSPACE)?;
    add_constant!(m, KEY_TAB)?;
    add_constant!(m, KEY_SPACE)?;
    add_constant!(m, KEY_MINUS)?;
    add_constant!(m, KEY_EQUALS)?;
    add_constant!(m, KEY_LEFTBRACKET)?;
    add_constant!(m, KEY_RIGHTBRACKET)?;
    add_constant!(m, KEY_BACKSLASH)?;
    add_constant!(m, KEY_NONUSHASH)?;
    add_constant!(m, KEY_SEMICOLON)?;
    add_constant!(m, KEY_APOSTROPHE)?;
    add_constant!(m, KEY_GRAVE)?;
    add_constant!(m, KEY_COMMA)?;
    add_constant!(m, KEY_PERIOD)?;
    add_constant!(m, KEY_SLASH)?;
    add_constant!(m, KEY_CAPSLOCK)?;
    add_constant!(m, KEY_F1)?;
    add_constant!(m, KEY_F2)?;
    add_constant!(m, KEY_F3)?;
    add_constant!(m, KEY_F4)?;
    add_constant!(m, KEY_F5)?;
    add_constant!(m, KEY_F6)?;
    add_constant!(m, KEY_F7)?;
    add_constant!(m, KEY_F8)?;
    add_constant!(m, KEY_F9)?;
    add_constant!(m, KEY_F10)?;
    add_constant!(m, KEY_F11)?;
    add_constant!(m, KEY_F12)?;
    add_constant!(m, KEY_PRINTSCREEN)?;
    add_constant!(m, KEY_SCROLLLOCK)?;
    add_constant!(m, KEY_PAUSE)?;
    add_constant!(m, KEY_INSERT)?;
    add_constant!(m, KEY_HOME)?;
    add_constant!(m, KEY_PAGEUP)?;
    add_constant!(m, KEY_DELETE)?;
    add_constant!(m, KEY_END)?;
    add_constant!(m, KEY_PAGEDOWN)?;
    add_constant!(m, KEY_RIGHT)?;
    add_constant!(m, KEY_LEFT)?;
    add_constant!(m, KEY_DOWN)?;
    add_constant!(m, KEY_UP)?;
    add_constant!(m, KEY_NUMLOCKCLEAR)?;
    add_constant!(m, KEY_KP_DIVIDE)?;
    add_constant!(m, KEY_KP_MULTIPLY)?;
    add_constant!(m, KEY_KP_MINUS)?;
    add_constant!(m, KEY_KP_PLUS)?;
    add_constant!(m, KEY_KP_ENTER)?;
    add_constant!(m, KEY_KP_1)?;
    add_constant!(m, KEY_KP_2)?;
    add_constant!(m, KEY_KP_3)?;
    add_constant!(m, KEY_KP_4)?;
    add_constant!(m, KEY_KP_5)?;
    add_constant!(m, KEY_KP_6)?;
    add_constant!(m, KEY_KP_7)?;
    add_constant!(m, KEY_KP_8)?;
    add_constant!(m, KEY_KP_9)?;
    add_constant!(m, KEY_KP_0)?;
    add_constant!(m, KEY_KP_PERIOD)?;
    add_constant!(m, KEY_NONUSBACKSLASH)?;
    add_constant!(m, KEY_APPLICATION)?;
    add_constant!(m, KEY_POWER)?;
    add_constant!(m, KEY_KP_EQUALS)?;
    add_constant!(m, KEY_F13)?;
    add_constant!(m, KEY_F14)?;
    add_constant!(m, KEY_F15)?;
    add_constant!(m, KEY_F16)?;
    add_constant!(m, KEY_F17)?;
    add_constant!(m, KEY_F18)?;
    add_constant!(m, KEY_F19)?;
    add_constant!(m, KEY_F20)?;
    add_constant!(m, KEY_F21)?;
    add_constant!(m, KEY_F22)?;
    add_constant!(m, KEY_F23)?;
    add_constant!(m, KEY_F24)?;
    add_constant!(m, KEY_EXECUTE)?;
    add_constant!(m, KEY_HELP)?;
    add_constant!(m, KEY_MENU)?;
    add_constant!(m, KEY_SELECT)?;
    add_constant!(m, KEY_STOP)?;
    add_constant!(m, KEY_AGAIN)?;
    add_constant!(m, KEY_UNDO)?;
    add_constant!(m, KEY_CUT)?;
    add_constant!(m, KEY_COPY)?;
    add_constant!(m, KEY_PASTE)?;
    add_constant!(m, KEY_FIND)?;
    add_constant!(m, KEY_MUTE)?;
    add_constant!(m, KEY_VOLUMEUP)?;
    add_constant!(m, KEY_VOLUMEDOWN)?;
    add_constant!(m, KEY_KP_COMMA)?;
    add_constant!(m, KEY_KP_EQUALSAS400)?;
    add_constant!(m, KEY_INTERNATIONAL1)?;
    add_constant!(m, KEY_INTERNATIONAL2)?;
    add_constant!(m, KEY_INTERNATIONAL3)?;
    add_constant!(m, KEY_INTERNATIONAL4)?;
    add_constant!(m, KEY_INTERNATIONAL5)?;
    add_constant!(m, KEY_INTERNATIONAL6)?;
    add_constant!(m, KEY_INTERNATIONAL7)?;
    add_constant!(m, KEY_INTERNATIONAL8)?;
    add_constant!(m, KEY_INTERNATIONAL9)?;
    add_constant!(m, KEY_LANG1)?;
    add_constant!(m, KEY_LANG2)?;
    add_constant!(m, KEY_LANG3)?;
    add_constant!(m, KEY_LANG4)?;
    add_constant!(m, KEY_LANG5)?;
    add_constant!(m, KEY_LANG6)?;
    add_constant!(m, KEY_LANG7)?;
    add_constant!(m, KEY_LANG8)?;
    add_constant!(m, KEY_LANG9)?;
    add_constant!(m, KEY_ALTERASE)?;
    add_constant!(m, KEY_SYSREQ)?;
    add_constant!(m, KEY_CANCEL)?;
    add_constant!(m, KEY_CLEAR)?;
    add_constant!(m, KEY_PRIOR)?;
    add_constant!(m, KEY_RETURN2)?;
    add_constant!(m, KEY_SEPARATOR)?;
    add_constant!(m, KEY_OUT)?;
    add_constant!(m, KEY_OPER)?;
    add_constant!(m, KEY_CLEARAGAIN)?;
    add_constant!(m, KEY_CRSEL)?;
    add_constant!(m, KEY_EXSEL)?;
    add_constant!(m, KEY_KP_00)?;
    add_constant!(m, KEY_KP_000)?;
    add_constant!(m, KEY_THOUSANDSSEPARATOR)?;
    add_constant!(m, KEY_DECIMALSEPARATOR)?;
    add_constant!(m, KEY_CURRENCYUNIT)?;
    add_constant!(m, KEY_CURRENCYSUBUNIT)?;
    add_constant!(m, KEY_KP_LEFTPAREN)?;
    add_constant!(m, KEY_KP_RIGHTPAREN)?;
    add_constant!(m, KEY_KP_LEFTBRACE)?;
    add_constant!(m, KEY_KP_RIGHTBRACE)?;
    add_constant!(m, KEY_KP_TAB)?;
    add_constant!(m, KEY_KP_BACKSPACE)?;
    add_constant!(m, KEY_KP_A)?;
    add_constant!(m, KEY_KP_B)?;
    add_constant!(m, KEY_KP_C)?;
    add_constant!(m, KEY_KP_D)?;
    add_constant!(m, KEY_KP_E)?;
    add_constant!(m, KEY_KP_F)?;
    add_constant!(m, KEY_KP_XOR)?;
    add_constant!(m, KEY_KP_POWER)?;
    add_constant!(m, KEY_KP_PERCENT)?;
    add_constant!(m, KEY_KP_LESS)?;
    add_constant!(m, KEY_KP_GREATER)?;
    add_constant!(m, KEY_KP_AMPERSAND)?;
    add_constant!(m, KEY_KP_DBLAMPERSAND)?;
    add_constant!(m, KEY_KP_VERTICALBAR)?;
    add_constant!(m, KEY_KP_DBLVERTICALBAR)?;
    add_constant!(m, KEY_KP_COLON)?;
    add_constant!(m, KEY_KP_HASH)?;
    add_constant!(m, KEY_KP_SPACE)?;
    add_constant!(m, KEY_KP_AT)?;
    add_constant!(m, KEY_KP_EXCLAM)?;
    add_constant!(m, KEY_KP_MEMSTORE)?;
    add_constant!(m, KEY_KP_MEMRECALL)?;
    add_constant!(m, KEY_KP_MEMCLEAR)?;
    add_constant!(m, KEY_KP_MEMADD)?;
    add_constant!(m, KEY_KP_MEMSUBTRACT)?;
    add_constant!(m, KEY_KP_MEMMULTIPLY)?;
    add_constant!(m, KEY_KP_MEMDIVIDE)?;
    add_constant!(m, KEY_KP_PLUSMINUS)?;
    add_constant!(m, KEY_KP_CLEAR)?;
    add_constant!(m, KEY_KP_CLEARENTRY)?;
    add_constant!(m, KEY_KP_BINARY)?;
    add_constant!(m, KEY_KP_OCTAL)?;
    add_constant!(m, KEY_KP_DECIMAL)?;
    add_constant!(m, KEY_KP_HEXADECIMAL)?;
    add_constant!(m, KEY_LCTRL)?;
    add_constant!(m, KEY_LSHIFT)?;
    add_constant!(m, KEY_LALT)?;
    add_constant!(m, KEY_LGUI)?;
    add_constant!(m, KEY_RCTRL)?;
    add_constant!(m, KEY_RSHIFT)?;
    add_constant!(m, KEY_RALT)?;
    add_constant!(m, KEY_RGUI)?;
    add_constant!(m, KEY_MODE)?;
    add_constant!(m, KEY_AUDIONEXT)?;
    add_constant!(m, KEY_AUDIOPREV)?;
    add_constant!(m, KEY_AUDIOSTOP)?;
    add_constant!(m, KEY_AUDIOPLAY)?;
    add_constant!(m, KEY_AUDIOMUTE)?;
    add_constant!(m, KEY_MEDIASELECT)?;
    add_constant!(m, KEY_WWW)?;
    add_constant!(m, KEY_MAIL)?;
    add_constant!(m, KEY_CALCULATOR)?;
    add_constant!(m, KEY_COMPUTER)?;
    add_constant!(m, KEY_AC_SEARCH)?;
    add_constant!(m, KEY_AC_HOME)?;
    add_constant!(m, KEY_AC_BACK)?;
    add_constant!(m, KEY_AC_FORWARD)?;
    add_constant!(m, KEY_AC_STOP)?;
    add_constant!(m, KEY_AC_REFRESH)?;
    add_constant!(m, KEY_AC_BOOKMARKS)?;
    add_constant!(m, KEY_BRIGHTNESSDOWN)?;
    add_constant!(m, KEY_BRIGHTNESSUP)?;
    add_constant!(m, KEY_DISPLAYSWITCH)?;
    add_constant!(m, KEY_KBDILLUMTOGGLE)?;
    add_constant!(m, KEY_KBDILLUMDOWN)?;
    add_constant!(m, KEY_KBDILLUMUP)?;
    add_constant!(m, KEY_EJECT)?;
    add_constant!(m, KEY_SLEEP)?;
    add_constant!(m, KEY_APP1)?;
    add_constant!(m, KEY_APP2)?;
    add_constant!(m, KEY_AUDIOREWIND)?;
    add_constant!(m, KEY_AUDIOFASTFORWARD)?;

    add_constant!(m, KEY_SHIFT)?;
    add_constant!(m, KEY_CTRL)?;
    add_constant!(m, KEY_ALT)?;
    add_constant!(m, KEY_GUI)?;

    add_constant!(m, MOUSE_POS_X)?;
    add_constant!(m, MOUSE_POS_Y)?;
    add_constant!(m, MOUSE_WHEEL_X)?;
    add_constant!(m, MOUSE_WHEEL_Y)?;

    add_constant!(m, MOUSE_BUTTON_LEFT)?;
    add_constant!(m, MOUSE_BUTTON_MIDDLE)?;
    add_constant!(m, MOUSE_BUTTON_RIGHT)?;
    add_constant!(m, MOUSE_BUTTON_X1)?;
    add_constant!(m, MOUSE_BUTTON_X2)?;
    add_constant!(m, MOUSE_BUTTON_UNKOWN)?;

    add_constant!(m, GAMEPAD1_AXIS_LEFTX)?;
    add_constant!(m, GAMEPAD1_AXIS_LEFTY)?;
    add_constant!(m, GAMEPAD1_AXIS_RIGHTX)?;
    add_constant!(m, GAMEPAD1_AXIS_RIGHTY)?;
    add_constant!(m, GAMEPAD1_AXIS_TRIGGERLEFT)?;
    add_constant!(m, GAMEPAD1_AXIS_TRIGGERRIGHT)?;

    add_constant!(m, GAMEPAD1_BUTTON_A)?;
    add_constant!(m, GAMEPAD1_BUTTON_B)?;
    add_constant!(m, GAMEPAD1_BUTTON_X)?;
    add_constant!(m, GAMEPAD1_BUTTON_Y)?;
    add_constant!(m, GAMEPAD1_BUTTON_BACK)?;
    add_constant!(m, GAMEPAD1_BUTTON_GUIDE)?;
    add_constant!(m, GAMEPAD1_BUTTON_START)?;
    add_constant!(m, GAMEPAD1_BUTTON_LEFTSTICK)?;
    add_constant!(m, GAMEPAD1_BUTTON_RIGHTSTICK)?;
    add_constant!(m, GAMEPAD1_BUTTON_LEFTSHOULDER)?;
    add_constant!(m, GAMEPAD1_BUTTON_RIGHTSHOULDER)?;
    add_constant!(m, GAMEPAD1_BUTTON_DPAD_UP)?;
    add_constant!(m, GAMEPAD1_BUTTON_DPAD_DOWN)?;
    add_constant!(m, GAMEPAD1_BUTTON_DPAD_LEFT)?;
    add_constant!(m, GAMEPAD1_BUTTON_DPAD_RIGHT)?;

    add_constant!(m, GAMEPAD2_AXIS_LEFTX)?;
    add_constant!(m, GAMEPAD2_AXIS_LEFTY)?;
    add_constant!(m, GAMEPAD2_AXIS_RIGHTX)?;
    add_constant!(m, GAMEPAD2_AXIS_RIGHTY)?;
    add_constant!(m, GAMEPAD2_AXIS_TRIGGERLEFT)?;
    add_constant!(m, GAMEPAD2_AXIS_TRIGGERRIGHT)?;

    add_constant!(m, GAMEPAD2_BUTTON_A)?;
    add_constant!(m, GAMEPAD2_BUTTON_B)?;
    add_constant!(m, GAMEPAD2_BUTTON_X)?;
    add_constant!(m, GAMEPAD2_BUTTON_Y)?;
    add_constant!(m, GAMEPAD2_BUTTON_BACK)?;
    add_constant!(m, GAMEPAD2_BUTTON_GUIDE)?;
    add_constant!(m, GAMEPAD2_BUTTON_START)?;
    add_constant!(m, GAMEPAD2_BUTTON_LEFTSTICK)?;
    add_constant!(m, GAMEPAD2_BUTTON_RIGHTSTICK)?;
    add_constant!(m, GAMEPAD2_BUTTON_LEFTSHOULDER)?;
    add_constant!(m, GAMEPAD2_BUTTON_RIGHTSHOULDER)?;
    add_constant!(m, GAMEPAD2_BUTTON_DPAD_UP)?;
    add_constant!(m, GAMEPAD2_BUTTON_DPAD_DOWN)?;
    add_constant!(m, GAMEPAD2_BUTTON_DPAD_LEFT)?;
    add_constant!(m, GAMEPAD2_BUTTON_DPAD_RIGHT)?;

    Ok(())
}
