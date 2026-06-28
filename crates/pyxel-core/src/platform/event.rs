use super::key::{Key, KeyValue};

pub enum Event {
    // Window visibility
    WindowShown,
    WindowHidden,

    // Key and control values
    KeyPressed { key: Key },
    KeyReleased { key: Key },
    KeyValueChanged { key: Key, value: KeyValue },

    // Text and file input
    TextInput { text: String },
    FileDropped { filename: String },

    // Application shutdown
    Quit,
}
