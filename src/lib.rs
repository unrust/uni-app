#![recursion_limit = "512"]

// wasm-unknown-unknown
#[cfg(target_arch = "wasm32")]
#[path = "web_app.rs"]
pub mod sys;

#[cfg(target_arch = "wasm32")]
#[path = "web_fs.rs"]
pub mod fs;

// NOT wasm-unknown-unknown
#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
extern crate time;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_app.rs"]
/// main application struct
pub mod sys;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_fs.rs"]
/// filesystem api
pub mod fs;

pub use self::fs::*;
pub use self::sys::*;

/// game window configuration
pub struct AppConfig {
    /// the window title (only visible on native target)
    pub title: String,
    /// the window/canvas size in pixels
    pub size: (u32, u32),
    /// sync frames with screen frequency (can only be disabled on native target)
    pub vsync: bool,
    /// start the program without actually creating a window, for test purposes
    pub headless: bool,
    /// start in full screen (native target only)
    pub fullscreen: bool,
    /// whether user can resize the window (native target only)
    pub resizable: bool,
    /// whether the mouse cursor is visible while in the window
    pub show_cursor: bool,
    /// whether clicking on the window close button exits the program or sends a CloseRequested event
    pub intercept_close_request: bool,
}

impl AppConfig {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32)) -> AppConfig {
        AppConfig {
            title: title.into(),
            size,
            vsync: true,
            headless: false,
            fullscreen: false,
            resizable: true,
            show_cursor: true,
            intercept_close_request: false,
        }
    }
}

/// keyboard and mouse events
pub mod events {
    use std::fmt;
    
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    /// keyboard key
    pub enum Key {
        /// The '1' key over the letters.
        Key1,
        /// The '2' key over the letters.
        Key2,
        /// The '3' key over the letters.
        Key3,
        /// The '4' key over the letters.
        Key4,
        /// The '5' key over the letters.
        Key5,
        /// The '6' key over the letters.
        Key6,
        /// The '7' key over the letters.
        Key7,
        /// The '8' key over the letters.
        Key8,
        /// The '9' key over the letters.
        Key9,
        /// The '0' key over the 'O' and 'P' keys.
        Key0,

        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,

        /// The Escape key, next to F1.
        Escape,

        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,

        /// Print Screen/SysRq.
        Snapshot,
        /// Scroll Lock.
        ScrollLock,
        /// Pause/Break key, next to Scroll lock.
        Pause,

        /// `Insert`, next to Backspace.
        Insert,
        Home,
        Delete,
        End,
        PageDown,
        PageUp,

        Left,
        Up,
        Right,
        Down,

        /// The Backspace key, right over Enter.
        Backspace,
        /// The Enter key.
        Enter,
        /// The space bar.
        Space,

        /// The "Compose" key on Linux.
        Compose,

        Caret,

        Numlock,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        NumpadAdd,
        NumpadDivide,
        NumpadDecimal,
        NumpadComma,
        NumpadEnter,
        NumpadEqual,
        NumpadMultiply,
        NumpadSubtract,

        Apostrophe,
        Asterisk,
        Backslash,
        CapsLock,
        Colon,
        Comma,
        Convert,
        Equal,
        Backquote,
        LAlt,
        LBracket,
        LCtrl,
        LShift,
        LWin,
        Command,
        Mail,
        MediaSelect,
        MediaStop,
        Minus,
        Mute,
        Period,
        Plus,
        RAlt,
        RBracket,
        RCtrl,
        RShift,
        RWin,
        Semicolon,
        Slash,
        Tab,
        Underline,
        Copy,
        Paste,
        Cut,

        Unknown,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    /// mouse button
    pub enum MouseButton {
        Left,
        Middle,
        Right,
        Other(usize),
    }

    #[derive(Debug, Clone)]
    /// data associated with a mouse button press/release event
    pub struct MouseButtonEvent {
        pub button: MouseButton,
    }

    #[derive(Clone)]
    /// data associated with a key press event
    /// Possible values for the scancode can be found in unrust/uni-app's `translate_scan_code`
    /// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
    /// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
    /// only exist on Mac.
    pub struct KeyDownEvent {
        /// scancode : top left letter is "KeyQ" even on an azerty keyboard
        pub code: String,
        /// virtual key code : top left letter is `Key::Q` on qwerty, `Key::A` on azerty
        pub key: Key,
        /// whether a shift key is pressed
        pub shift: bool,
        /// whether an alt key is pressed
        pub alt: bool,
        /// whether a control key is pressed
        pub ctrl: bool,
    }

    #[derive(Clone)]
    /// data associated with a key release event
    /// Possible values for the scancode can be found in unrust/uni-app's `translate_scan_code`
    /// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
    /// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
    /// only exist on Mac.
    pub struct KeyUpEvent {
        /// scancode : top left letter is "KeyQ" even on an azerty keyboard
        pub code: String,
        /// virtual key code : top left letter is `Key::Q` on qwerty, `Key::A` on azerty
        pub key: Key,
        /// whether a shift key is pressed
        pub shift: bool,
        /// whether an alt key is pressed
        pub alt: bool,
        /// whether a control key is pressed
        pub ctrl: bool,
    }

    impl fmt::Debug for KeyUpEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{} {} {} {} {:?}",
                if self.shift { "shift" } else { "" },
                if self.alt { "alt" } else { "" },
                if self.ctrl { "ctrl" } else { "" },
                self.code,
                self.key,
            )
        }
    }

    impl fmt::Debug for KeyDownEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{} {} {} {} {:?}",
                if self.shift { "shift" } else { "" },
                if self.alt { "alt" } else { "" },
                if self.ctrl { "ctrl" } else { "" },
                self.code,
                self.key,
            )
        }
    }
}

pub use events::*;

#[derive(Debug, Clone)]
/// window event types
pub enum AppEvent {
    /// mouse button press
    MouseDown(MouseButtonEvent),
    /// mouse button release
    MouseUp(MouseButtonEvent),
    /// keyboard press
    KeyDown(KeyDownEvent),
    /// keyboard release
    KeyUp(KeyUpEvent),
    /// text input events
    CharEvent(char),
    /// window resize
    Resized((u32, u32)),
    /// mouse cursor position in pixels from the window top-left
    MousePos((f64, f64)),
    /// a file has been dropped on the game window. Get it with `App.get_dropped_file`
    FileDropped(String),
    /// window close button was pressed and [`AppConfig.intercept_close_request`] is true
    CloseRequested,
}
