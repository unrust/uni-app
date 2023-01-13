use glutin::event::VirtualKeyCode;

use crate::events::Key;

pub fn translate_virtual_key(c: VirtualKeyCode) -> Key {
    match c {
        VirtualKeyCode::Key1 => Key::Key1,
        VirtualKeyCode::Key2 => Key::Key2,
        VirtualKeyCode::Key3 => Key::Key3,
        VirtualKeyCode::Key4 => Key::Key4,
        VirtualKeyCode::Key5 => Key::Key5,
        VirtualKeyCode::Key6 => Key::Key6,
        VirtualKeyCode::Key7 => Key::Key7,
        VirtualKeyCode::Key8 => Key::Key8,
        VirtualKeyCode::Key9 => Key::Key9,
        VirtualKeyCode::Key0 => Key::Key0,
        VirtualKeyCode::A => Key::A,
        VirtualKeyCode::B => Key::B,
        VirtualKeyCode::C => Key::C,
        VirtualKeyCode::D => Key::D,
        VirtualKeyCode::E => Key::E,
        VirtualKeyCode::F => Key::F,
        VirtualKeyCode::G => Key::G,
        VirtualKeyCode::H => Key::H,
        VirtualKeyCode::I => Key::I,
        VirtualKeyCode::J => Key::J,
        VirtualKeyCode::K => Key::K,
        VirtualKeyCode::L => Key::L,
        VirtualKeyCode::M => Key::M,
        VirtualKeyCode::N => Key::N,
        VirtualKeyCode::O => Key::O,
        VirtualKeyCode::P => Key::P,
        VirtualKeyCode::Q => Key::Q,
        VirtualKeyCode::R => Key::R,
        VirtualKeyCode::S => Key::S,
        VirtualKeyCode::T => Key::T,
        VirtualKeyCode::U => Key::U,
        VirtualKeyCode::V => Key::V,
        VirtualKeyCode::W => Key::W,
        VirtualKeyCode::X => Key::X,
        VirtualKeyCode::Y => Key::Y,
        VirtualKeyCode::Z => Key::Z,
        VirtualKeyCode::Escape => Key::Escape,
        VirtualKeyCode::F1 => Key::F1,
        VirtualKeyCode::F2 => Key::F2,
        VirtualKeyCode::F3 => Key::F3,
        VirtualKeyCode::F4 => Key::F4,
        VirtualKeyCode::F5 => Key::F5,
        VirtualKeyCode::F6 => Key::F6,
        VirtualKeyCode::F7 => Key::F7,
        VirtualKeyCode::F8 => Key::F8,
        VirtualKeyCode::F9 => Key::F9,
        VirtualKeyCode::F10 => Key::F10,
        VirtualKeyCode::F11 => Key::F11,
        VirtualKeyCode::F12 => Key::F12,
        VirtualKeyCode::F13 => Key::F13,
        VirtualKeyCode::F14 => Key::F14,
        VirtualKeyCode::F15 => Key::F15,
        VirtualKeyCode::F16 => Key::F16,
        VirtualKeyCode::F17 => Key::F17,
        VirtualKeyCode::F18 => Key::F18,
        VirtualKeyCode::F19 => Key::F19,
        VirtualKeyCode::F20 => Key::F20,
        VirtualKeyCode::F21 => Key::F21,
        VirtualKeyCode::F22 => Key::F22,
        VirtualKeyCode::F23 => Key::F23,
        VirtualKeyCode::F24 => Key::F24,
        VirtualKeyCode::Snapshot => Key::Snapshot,
        VirtualKeyCode::Scroll => Key::ScrollLock,
        VirtualKeyCode::Pause => Key::Pause,
        VirtualKeyCode::Insert => Key::Insert,
        VirtualKeyCode::Home => Key::Home,
        VirtualKeyCode::Delete => Key::Delete,
        VirtualKeyCode::End => Key::End,
        VirtualKeyCode::PageDown => Key::PageDown,
        VirtualKeyCode::PageUp => Key::PageUp,
        VirtualKeyCode::Left => Key::Left,
        VirtualKeyCode::Up => Key::Up,
        VirtualKeyCode::Right => Key::Right,
        VirtualKeyCode::Down => Key::Down,
        VirtualKeyCode::Back => Key::Backspace,
        VirtualKeyCode::Return => Key::Enter,
        VirtualKeyCode::Space => Key::Space,
        VirtualKeyCode::Compose => Key::Compose,
        VirtualKeyCode::Numlock => Key::Numlock,
        VirtualKeyCode::Numpad0 => Key::Numpad0,
        VirtualKeyCode::Numpad1 => Key::Numpad1,
        VirtualKeyCode::Numpad2 => Key::Numpad2,
        VirtualKeyCode::Numpad3 => Key::Numpad3,
        VirtualKeyCode::Numpad4 => Key::Numpad4,
        VirtualKeyCode::Numpad5 => Key::Numpad5,
        VirtualKeyCode::Numpad6 => Key::Numpad6,
        VirtualKeyCode::Numpad7 => Key::Numpad7,
        VirtualKeyCode::Numpad8 => Key::Numpad8,
        VirtualKeyCode::Numpad9 => Key::Numpad9,
        VirtualKeyCode::AbntC1 => Key::AbntC1,
        VirtualKeyCode::AbntC2 => Key::AbntC2,
        VirtualKeyCode::NumpadAdd => Key::NumpadAdd,
        VirtualKeyCode::Apostrophe => Key::Apostrophe,
        VirtualKeyCode::Apps => Key::Apps,
        VirtualKeyCode::Asterisk => Key::Asterisk,
        VirtualKeyCode::At => Key::At,
        VirtualKeyCode::Ax => Key::Ax,
        VirtualKeyCode::Backslash => Key::Backslash,
        VirtualKeyCode::Calculator => Key::Calculator,
        VirtualKeyCode::Capital => Key::CapsLock,
        VirtualKeyCode::Colon => Key::Colon,
        VirtualKeyCode::Comma => Key::Comma,
        VirtualKeyCode::Convert => Key::Convert,
        VirtualKeyCode::NumpadDecimal => Key::NumpadDecimal,
        VirtualKeyCode::NumpadDivide => Key::NumpadDivide,
        VirtualKeyCode::Equals => Key::Equal,
        VirtualKeyCode::Grave => Key::Backquote,
        VirtualKeyCode::Kana => Key::Kana,
        VirtualKeyCode::Kanji => Key::Kanji,
        VirtualKeyCode::LAlt => Key::LAlt,
        VirtualKeyCode::LBracket => Key::LBracket,
        VirtualKeyCode::LControl => Key::LControl,
        VirtualKeyCode::LShift => Key::LShift,

        #[cfg(not(target_os="macos"))]
        VirtualKeyCode::LWin => Key::LWin,
        #[cfg(target_os="macos")]
        VirtualKeyCode::LWin => Key::Command,

        VirtualKeyCode::Mail => Key::Mail,
        VirtualKeyCode::MediaSelect => Key::MediaSelect,
        VirtualKeyCode::MediaStop => Key::MediaStop,
        VirtualKeyCode::Minus => Key::Minus,
        VirtualKeyCode::NumpadMultiply => Key::NumpadMultiply,
        VirtualKeyCode::Mute => Key::Mute,
        VirtualKeyCode::MyComputer => Key::MyComputer,
        VirtualKeyCode::NavigateForward => Key::NavigateForward,
        VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
        VirtualKeyCode::NextTrack => Key::NextTrack,
        VirtualKeyCode::NoConvert => Key::NoConvert,
        VirtualKeyCode::NumpadComma => Key::NumpadComma,
        VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
        VirtualKeyCode::NumpadEquals => Key::NumpadEqual,
        VirtualKeyCode::OEM102 => Key::OEM102,
        VirtualKeyCode::Period => Key::Period,
        VirtualKeyCode::PlayPause => Key::PlayPause,
        VirtualKeyCode::Power => Key::Power,
        VirtualKeyCode::Plus => Key::Plus,
        VirtualKeyCode::PrevTrack => Key::PrevTrack,
        VirtualKeyCode::RAlt => Key::RAlt,
        VirtualKeyCode::RBracket => Key::RBracket,
        VirtualKeyCode::RControl => Key::RControl,
        VirtualKeyCode::RShift => Key::RShift,
        VirtualKeyCode::RWin => Key::RWin,
        VirtualKeyCode::Semicolon => Key::Semicolon,
        VirtualKeyCode::Slash => Key::Slash,
        VirtualKeyCode::Sleep => Key::Sleep,
        VirtualKeyCode::Stop => Key::Stop,
        VirtualKeyCode::NumpadSubtract => Key::NumpadSubtract,
        VirtualKeyCode::Sysrq => Key::Sysrq,
        VirtualKeyCode::Tab => Key::Tab,
        VirtualKeyCode::Underline => Key::Underline,
        VirtualKeyCode::Unlabeled => Key::Unlabeled,
        VirtualKeyCode::VolumeDown => Key::VolumeDown,
        VirtualKeyCode::VolumeUp => Key::VolumeUp,
        VirtualKeyCode::Wake => Key::Wake,
        VirtualKeyCode::WebBack => Key::WebBack,
        VirtualKeyCode::WebFavorites => Key::WebFavorites,
        VirtualKeyCode::WebForward => Key::WebForward,
        VirtualKeyCode::WebHome => Key::WebHome,
        VirtualKeyCode::WebRefresh => Key::WebRefresh,
        VirtualKeyCode::WebSearch => Key::WebSearch,
        VirtualKeyCode::WebStop => Key::WebStop,
        VirtualKeyCode::Yen => Key::Yen,
        VirtualKeyCode::Caret => Key::Caret,
        VirtualKeyCode::Copy => Key::Copy,
        VirtualKeyCode::Paste => Key::Paste,
        VirtualKeyCode::Cut => Key::Cut,
    }
}

#[cfg(target_os = "macos")]
pub fn translate_scan_code(c: u32) -> &'static str {
    match c {
        0x12 => Key::Key1,
        0x13 => Key::Key2,
        0x14 => Key::Key3,
        0x15 => Key::Key4,
        0x17 => Key::Key5,
        0x16 => Key::Key6,
        0x1A => Key::Key7,
        0x1C => Key::Key8,
        0x19 => Key::Key9,
        0x1D => Key::Key0,
        0x00 => Key::A,
        0x0B => Key::B,
        0x08 => Key::C,
        0x02 => Key::D,
        0x0E => Key::E,
        0x03 => Key::F,
        0x05 => Key::G,
        0x04 => Key::H,
        0x22 => Key::I,
        0x26 => Key::J,
        0x28 => Key::K,
        0x25 => Key::L,
        0x2E => Key::M,
        0x2D => Key::N,
        0x1F => Key::O,
        0x23 => Key::P,
        0x0C => Key::Q,
        0x0F => Key::R,
        0x01 => Key::S,
        0x11 => Key::T,
        0x20 => Key::U,
        0x09 => Key::V,
        0x0D => Key::W,
        0x07 => Key::X,
        0x10 => Key::Y,
        0x06 => Key::Z,
        0x35 => Key::Escape,
        0x7A => Key::F1,
        0x78 => Key::F2,
        0x63 => Key::F3,
        0x76 => Key::F4,
        0x60 => Key::F5,
        0x61 => Key::F6,
        0x62 => Key::F7,
        0x64 => Key::F8,
        0x65 => Key::F9,
        0x6D => Key::F10,
        0x67 => Key::F11,
        0x6F => Key::F12,
        0x69 => Key::F13,
        0x6B => Key::F14,
        0x71 => Key::F15,
        // 0x37 => Key::Snapshot,
        // 0x46 => Key::ScrollLock,
        // Pause => Key::Pause,
        // 0x52 => Key::Insert,
        0x73 => Key::Home,
        0x75 => Key::Delete,
        0x77 => Key::End,
        0x79 => Key::PageDown,
        0x74 => Key::PageUp,
        0x7B => Key::Left,
        0x7E => Key::Up,
        0x7C => Key::Right,
        0x7D => Key::Down,
        0x33 => Key::Backspace,
        0x24 => Key::Enter,
        0x31 => Key::Space,
        // Compose => Key::Unknown,
        0x47 => Key::Numlock,
        0x52 => Key::Numpad0,
        0x53 => Key::Numpad1,
        0x54 => Key::Numpad2,
        0x55 => Key::Numpad3,
        0x56 => Key::Numpad4,
        0x57 => Key::Numpad5,
        0x58 => Key::Numpad6,
        0x59 => Key::Numpad7,
        0x5B => Key::Numpad8,
        0x5C => Key::Numpad9,
        // AbntC1 => Key::Unknown,
        // AbntC2 => Key::Unknown,
        0x45 => Key::NumpadAdd,
        0x27 => Key::Apostrophe,
        // Apps => Key::Unknown,
        // At => Key::Unknown,
        // Ax => Key::Unknown,
        0x2A => Key::Backslash,
        // Calculator => Key::Unknown,
        0x39 => Key::CapsLock,
        0x29 => Key::Colon,
        0x2B => Key::Comma,
        // Convert => Key::Unknown,
        0x41 => Key::NumpadDecimal,
        0x4B => Key::NumpadDivide,
        0x18 => Key::Equal,
        0x32 => Key::Backquote,
        // Kana => Key::Unknown,
        // Kanji => Key::Unknown,
        // LAlt => Key::Unknown,
        0x21 => Key::LBracket,
        0x3B => Key::LControl,
        // 0x38 => Key::AltLeft,
        0x38 => Key::LShift,
        0x37 => Key::Command,
        // 0x5B => Key::LeftWin,
        // Mail => Key::Unknown,
        // MediaSelect => Key::Unknown,
        // MediaStop => Key::Unknown,
        0x1B => Key::Minus,
        0x43 => Key::NumpadMultiply,
        // Mute => Key::Unknown,
        // MyComputer => Key::Unknown,
        // NavigateForward => Key::Unknown,
        // NavigateBackward => Key::Unknown,
        // NextTrack => Key::Unknown,
        // NoConvert => Key::Unknown,
        // NumpadComma => Key::NumpadComma,
        0x4C => Key::NumpadEnter,
        0x51 => Key::NumpadEqual,
        // OEM102 => Key::Unknown,
        0x2F => Key::Period,
        // PlayPause => Key::Unknown,
        // Power => Key::Unknown,
        // PrevTrack => Key::Unknown,
        // RAlt => Key::Unknown,
        0x1E => Key::RBracket,
        // RControl => Key::RControl, // Same as LControl
        // RMenu => Key::RAlt, // Same as LControl
        // 0x36 => Key::RShift,
        // RWin => Key::Unknown,
        // 0x27 => Key::Semicolon,
        0x2C => Key::Slash,
        // Sleep => Key::Unknown,
        // Stop => Key::Unknown,
        0x4E => Key::NumpadSubtract,
        // Sysrq => Key::Unknown,
        0x30 => Key::Tab,
        // Underline => Key::Unknown,
        // Unlabeled => Key::Unknown,
        // VolumeDown => Key::Unknown,
        // VolumeUp => Key::Unknown,
        // Wake => Key::Unknown,
        // WebBack => Key::Unknown,
        // WebFavorites => Key::Unknown,
        // WebForward => Key::Unknown,
        // WebHome => Key::Unknown,
        // WebRefresh => Key::Unknown,
        // WebSearch => Key::Unknown,
        // WebStop => Key::Unknown,
        // Yen => Key::Unknown,
        // Caret => Key::Caret,
        _ => Key::Unknown,
    }
}

#[cfg(target_os = "windows")]
pub fn translate_scan_code(c: u32) -> Key {
    match c {
        0x02 => Key::Key0,
        0x03 => Key::Key1,
        0x04 => Key::Key2,
        0x05 => Key::Key3,
        0x06 => Key::Key4,
        0x07 => Key::Key5,
        0x08 => Key::Key6,
        0x09 => Key::Key7,
        0x0A => Key::Key8,
        0x0B => Key::Key9,
        0x1E => Key::A,
        0x30 => Key::B,
        0x2E => Key::C,
        0x20 => Key::D,
        0x12 => Key::E,
        0x21 => Key::F,
        0x22 => Key::G,
        0x23 => Key::H,
        0x17 => Key::I,
        0x24 => Key::J,
        0x25 => Key::K,
        0x26 => Key::L,
        0x32 => Key::M,
        0x31 => Key::N,
        0x18 => Key::O,
        0x19 => Key::P,
        0x10 => Key::Q,
        0x13 => Key::R,
        0x1F => Key::S,
        0x14 => Key::T,
        0x16 => Key::U,
        0x2F => Key::V,
        0x11 => Key::W,
        0x2D => Key::X,
        0x15 => Key::Y,
        0x2C => Key::Z,
        0x01 => Key::Escape,
        0x3B => Key::F1,
        0x3C => Key::F2,
        0x3D => Key::F3,
        0x3E => Key::F4,
        0x3F => Key::F5,
        0x40 => Key::F6,
        0x41 => Key::F7,
        0x42 => Key::F8,
        0x43 => Key::F9,
        0x44 => Key::F10,
        0x85 => Key::F11,
        0x86 => Key::F12,
        // F13 => Key::F13,
        // F14 => Key::F14,
        // F15 => Key::F15,
        0x37 => Key::Snapshot,
        0x46 => Key::ScrollLock,
        // Pause => Key::Pause,
        0x52 => Key::Insert,
        0x47 => Key::Home,
        0x53 => Key::Delete,
        0x4F => Key::End,
        0x51 => Key::PageDown,
        0x49 => Key::PageUp,
        0x4B => Key::Left,
        0x48 => Key::Up,
        0x4D => Key::Right,
        0x50 => Key::Down,
        0x0E => Key::Backspace,
        0x1C => Key::Enter,
        0x39 => Key::Space,
        // Compose => Key::Unknown,
        0x45 => Key::Numlock,
        // Numpad0 => Key::Numpad0, // Same as Insert
        // Numpad1 => Key::Numpad1, // Same as End
        // Numpad2 => Key::Numpad2, // Same as Down
        // Numpad3 => Key::Numpad3, // Same as PageDown
        // Numpad4 => Key::Numpad4, // Same as Left
        0x4C => Key::Numpad5,
        // Numpad6 => Key::Numpad6, // Same as Right
        // Numpad7 => Key::Numpad7, // Same as Home
        // Numpad8 => Key::Numpad8, // Same as Up
        // Numpad9 => Key::Numpad9, // Same as PageUp
        // AbntC1 => Key::Unknown,
        // AbntC2 => Key::Unknown,
        0x4E => Key::NumpadAdd,
        // Apostrophe => Key::Apostrophe,
        // Apps => Key::Unknown,
        // At => Key::Unknown,
        // Ax => Key::Unknown,
        0x2B => Key::Backslash,
        // Calculator => Key::Unknown,
        0x3A => Key::CapsLock,
        0x27 => Key::Colon,
        0x33 => Key::Comma,
        // Convert => Key::Unknown,
        // Decimal => Key::NumpadDecimal, // Same as Delete
        0x35 => Key::NumpadDivide,
        0x0D => Key::Equal,
        0x29 => Key::Backquote,
        // Kana => Key::Unknown,
        // Kanji => Key::Unknown,
        // LAlt => Key::Unknown,
        0x1A => Key::LBracket,
        0x1D => Key::LControl,
        0x38 => Key::RControl,
        0x2A => Key::LShift,
        0x5B => Key::LWin,
        // Mail => Key::Unknown,
        // MediaSelect => Key::Unknown,
        // MediaStop => Key::Unknown,
        0x0C => Key::Minus,
        // Multiply => Key::NumpadMultiply, // Same as Snapshot
        // Mute => Key::Unknown,
        // MyComputer => Key::Unknown,
        // NavigateForward => Key::Unknown,
        // NavigateBackward => Key::Unknown,
        // NextTrack => Key::Unknown,
        // NoConvert => Key::Unknown,
        // NumpadComma => Key::NumpadComma,
        // NumpadEnter => Key::NumpadEnter, // Same as Enter
        // NumpadEquals => Key::NumpadEqual,
        // OEM102 => Key::Unknown,
        0x34 => Key::Period,
        // PlayPause => Key::Unknown,
        // Power => Key::Unknown,
        // PrevTrack => Key::Unknown,
        // RAlt => Key::Unknown,
        0x1B => Key::RBracket,
        // RControl => Key::RControl, // Same as LControl
        // RMenu => Key::RAlt, // Same as LControl
        0x36 => Key::RShift,
        // RWin => Key::Unknown,
        // 0x27 => Key::Semicolon,
        // 0x35 => Key::Slash, // Same as NumpadDivide
        // Sleep => Key::Unknown,
        // Stop => Key::Unknown,
        0x4A => Key::NumpadSubtract,
        // Sysrq => Key::Unknown,
        0x0F => Key::Tab,
        // Underline => Key::Unknown,
        // Unlabeled => Key::Unknown,
        // VolumeDown => Key::Unknown,
        // VolumeUp => Key::Unknown,
        // Wake => Key::Unknown,
        // WebBack => Key::Unknown,
        // WebFavorites => Key::Unknown,
        // WebForward => Key::Unknown,
        // WebHome => Key::Unknown,
        // WebRefresh => Key::Unknown,
        // WebSearch => Key::Unknown,
        // WebStop => Key::Unknown,
        // Yen => Key::Unknown,
        // Caret => Key::Caret,
        _ => Key::Unknown,
    }
}

#[cfg(target_os = "linux")]
pub fn translate_scan_code(c: u32) -> Key {
    match c {
        0x02 => Key::Key0,
        0x03 => Key::Key1,
        0x04 => Key::Key2,
        0x05 => Key::Key3,
        0x06 => Key::Key4,
        0x07 => Key::Key5,
        0x08 => Key::Key6,
        0x09 => Key::Key7,
        0x0A => Key::Key8,
        0x0B => Key::Key9,
        0x1E => Key::A,
        0x30 => Key::B,
        0x2E => Key::C,
        0x20 => Key::D,
        0x12 => Key::E,
        0x21 => Key::F,
        0x22 => Key::G,
        0x23 => Key::H,
        0x17 => Key::I,
        0x24 => Key::J,
        0x25 => Key::K,
        0x26 => Key::L,
        0x32 => Key::M,
        0x31 => Key::N,
        0x18 => Key::O,
        0x19 => Key::P,
        0x10 => Key::Q,
        0x13 => Key::R,
        0x1F => Key::S,
        0x14 => Key::T,
        0x16 => Key::U,
        0x2F => Key::V,
        0x11 => Key::W,
        0x2D => Key::X,
        0x15 => Key::Y,
        0x2C => Key::Z,
        0x01 => Key::Escape,
        0x3B => Key::F1,
        0x3C => Key::F2,
        0x3D => Key::F3,
        0x3E => Key::F4,
        0x3F => Key::F5,
        0x40 => Key::F6,
        0x41 => Key::F7,
        0x42 => Key::F8,
        0x43 => Key::F9,
        0x44 => Key::F10,
        0x85 => Key::F11,
        0x86 => Key::F12,
        // F13 => Key::F13,
        // F14 => Key::F14,
        // F15 => Key::F15,
        0x37 => Key::Snapshot,
        0x46 => Key::ScrollLock,
        // Pause => Key::Pause,
        0x52 => Key::Insert,
        0x47 => Key::Home,
        0x53 => Key::Delete,
        0x4F => Key::End,
        0x6D => Key::PageDown,
        0x68 => Key::PageUp,
        0x69 => Key::Left,
        0x67 => Key::Up,
        0x6a => Key::Right,
        0x6c => Key::Down,
        0x0E => Key::Backspace,
        0x1C => Key::Enter,
        0x39 => Key::Space,
        // Compose => Key::Unknown,
        0x45 => Key::Numlock,
        // Numpad0 => Key::Numpad0, // Same as Insert
        // Numpad1 => Key::Numpad1, // Same as End
        // Numpad2 => Key::Numpad2, // Same as Down
        // Numpad3 => Key::Numpad3, // Same as PageDown
        // Numpad4 => Key::Numpad4, // Same as Left
        0x4C => Key::Numpad5,
        // Numpad6 => Key::Numpad6, // Same as Right
        // Numpad7 => Key::Numpad7, // Same as Home
        // Numpad8 => Key::Numpad8, // Same as Up
        // Numpad9 => Key::Numpad9, // Same as PageUp
        // AbntC1 => Key::Unknown,
        // AbntC2 => Key::Unknown,
        0x4E => Key::NumpadAdd,
        // Apostrophe => Key::Apostrophe,
        // Apps => Key::Unknown,
        // At => Key::Unknown,
        // Ax => Key::Unknown,
        0x2B => Key::Backslash,
        // Calculator => Key::Unknown,
        0x3A => Key::CapsLock,
        0x27 => Key::Colon,
        0x33 => Key::Comma,
        // Convert => Key::Unknown,
        // Decimal => Key::NumpadDecimal, // Same as Delete
        0x35 => Key::NumpadDivide,
        0x0D => Key::Equal,
        0x29 => Key::Backquote,
        // Kana => Key::Unknown,
        // Kanji => Key::Unknown,
        // LAlt => Key::Unknown,
        0x1A => Key::LBracket,
        0x1D => Key::LControl,
        0x38 => Key::LAlt,
        0x2A => Key::LShift,
        0x5B => Key::LWin,
        // Mail => Key::Unknown,
        // MediaSelect => Key::Unknown,
        // MediaStop => Key::Unknown,
        0x0C => Key::Minus,
        // Multiply => Key::NumpadMultiply, // Same as Snapshot
        // Mute => Key::Unknown,
        // MyComputer => Key::Unknown,
        // NavigateForward => Key::Unknown,
        // NavigateBackward => Key::Unknown,
        // NextTrack => Key::Unknown,
        // NoConvert => Key::Unknown,
        // NumpadComma => Key::NumpadComma,
        // NumpadEnter => Key::NumpadEnter, // Same as Enter
        // NumpadEquals => Key::NumpadEqual,
        // OEM102 => Key::Unknown,
        0x34 => Key::Period,
        // PlayPause => Key::Unknown,
        // Power => Key::Unknown,
        // PrevTrack => Key::Unknown,
        // RAlt => Key::Unknown,
        0x1B => Key::RBracket,
        // RControl => Key::ControlRight, // Same as LControl
        // RMenu => Key::AltRight, // Same as LControl
        0x36 => Key::RShift,
        // RWin => Key::Unknown,
        // 0x27 => Key::Semicolon,
        // 0x35 => Key::Slash, // Same as NumpadDivide
        // Sleep => Key::Unknown,
        // Stop => Key::Unknown,
        0x4A => Key::NumpadSubtract,
        // Sysrq => Key::Unknown,
        0x0F => Key::Tab,
        // Underline => Key::Unknown,
        // Unlabeled => Key::Unknown,
        // VolumeDown => Key::Unknown,
        // VolumeUp => Key::Unknown,
        // Wake => Key::Unknown,
        // WebBack => Key::Unknown,
        // WebFavorites => Key::Unknown,
        // WebForward => Key::Unknown,
        // WebHome => Key::Unknown,
        // WebRefresh => Key::Unknown,
        // WebSearch => Key::Unknown,
        // WebStop => Key::Unknown,
        // Yen => Key::Unknown,
        // Caret => Key::Caret,
        _ => Key::Unknown,
    }
}
