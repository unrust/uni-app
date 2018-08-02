#![feature(nll)]
#![recursion_limit = "512"]

// wasm-unknown-unknown
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

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
        }
    }
}

/// keyboard and mouse events
pub mod events {
    use std::fmt;

    #[derive(Debug, Clone)]
    /// data associated with a mouse button press/release event
    pub struct MouseButtonEvent {
        /// the button number (0=left, 1=middle, 2=right, ...)
        pub button: usize,
    }

    #[derive(Clone)]
    /// data associated with a key press event
    /// Possible values for the scancode/virtual key code can be found in unrust/uni-app's `translate_scan_code`
    /// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
    /// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
    /// only exist on Mac.
    pub struct KeyDownEvent {
        /// scancode : top left letter is "KeyQ" even on an azerty keyboard
        pub code: String,
        /// virtual key code : top left letter is "KeyQ" on qwerty, "KeyA" on azerty
        pub key: String,
        /// whether a shift key is pressed
        pub shift: bool,
        /// whether an alt key is pressed
        pub alt: bool,
        /// whether a control key is pressed
        pub ctrl: bool,
    }

    #[derive(Clone)]
    /// data associated with a key release event
    /// Possible values for the scancode/virtual key code can be found in unrust/uni-app's `translate_scan_code`
    /// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
    /// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
    /// only exist on Mac.
    pub struct KeyUpEvent {
        /// scancode : top left letter is "KeyQ" even on an azerty keyboard
        pub code: String,
        /// virtual key code : top left letter is "KeyQ" on qwerty, "KeyA" on azerty
        pub key: String,
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
                "{} {} {} {} {}",
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
                "{} {} {} {} {}",
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
    /// window resize
    Resized((u32, u32)),
    /// mouse cursor position in pixels from the window top-left
    MousePos((f64, f64)),
}
