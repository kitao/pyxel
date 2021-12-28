pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
    Unknown,
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
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad,
}

pub enum Event {
    // System events
    Quit,
    DropFile {
        filename: String,
    },

    // Window events
    FocusGained,
    FocusLost,
    Maximized,
    Minimized,

    // Key events
    KeyDown {
        keycode: u32,
    },
    KeyUp {
        keycode: u32,
    },
    TextInput {
        text: String,
    },

    // Mouse events
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

    // Controller events
    ControllerAxisMotion {
        which: u32,
        axis: ControllerAxis,
        value: i32,
    },
    ControllerButtonDown {
        which: u32,
        button: ControllerButton,
    },
    ControllerButtonUp {
        which: u32,
        button: ControllerButton,
    },
}
