pub type Scancode = u16;

pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

pub enum ControllerAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

pub enum ControllerButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
}

pub enum Event {
    //
    // System Events
    //
    Quit,
    DropFile {
        file: String,
    },

    //
    // Window Events
    //
    WindowShown,
    WindowHidden,
    WindowMoved {
        x: i32,
        y: i32,
    },
    WindowResized {
        width: u32,
        height: u32,
    },
    WindowMinimized,
    WindowMaximized,
    WindowEnter,
    WindowLeave,
    WindowFocusGained,
    WindowFocusLost,
    WindowClose,

    //
    // Key Events
    //
    KeyDown {
        key: Scancode,
    },
    KeyUp {
        key: Scancode,
    },
    KeyInput {
        key: Vec<Scancode>,
    },

    //
    // Mouse Events
    //
    MouseMotion {
        x: i32,
        y: i32,
    },
    MouseButtonDown {
        button: MouseButton,
    },
    MouseButtonUp {
        button: MouseButton,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },

    //
    // Controller Events
    //
    ControllerAxisMotion {
        id: u8,
        axis: ControllerAxis,
        value: i32,
    },
    ControllerButtonDown {
        id: u8,
        button: ControllerButton,
    },
    ControllerButtonUp {
        id: u8,
        button: ControllerButton,
    },
}
