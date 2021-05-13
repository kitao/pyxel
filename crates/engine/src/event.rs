use crate::key::Key;

pub enum Event {
    Quit,
    DropFile,

    WindowShown,
    WindowHidden,
    WindowMoved { x: i32, y: i32 },
    WindowResized { width: u32, height: u32 },
    WindowMinimized,
    WindowMaximized,
    WindowEnter,
    WindowLeave,
    WindowFocusGained,
    WindowFocusLost,
    WindowClose,

    InputValue,
    InputDown,
    InputUp,
    InputText,
}
