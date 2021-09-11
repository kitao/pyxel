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
}

pub enum Event {
    //
    // System Events
    //
    Quit,
    DropFile {
        filename: String,
    },

    //
    // Key Events
    //
    KeyDown {
        key: u32,
    },
    KeyUp {
        key: u32,
    },
    TextInput {
        text: String,
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
