# Changelog

## [0.5.0] - 2023-03-30
### Added
- on native target, you can set the window icon with AppConfig.icon

## [0.4.1] - 2023-02-17
### Fixed
- scancodes for F11 and F12 (windows & linux)
- scancode for LAlt (windows)

## [0.4.0] - 2023-01-15
### Added
- filesystem example in basic.rs
- URL support in filesystem API on native targets when enabling the http feature

    This breaks API compatibility as File::is_ready(&self) becomes File::is_ready(&mut self)

## [0.3.1] - 2023-01-13
### Fixed
- fixed wasm target compilation

## [0.3.0] - 2023-01-13
### Changed
- keyboard and mouse API revamped KeyUp/DownEvent.code and MouseButtonEvent.button are now enums

### Fixed
- App:print use println on native target to have the same behavior as wasm

### Added
- support for VirtualKeyCode::Apostrophe on mac

## [0.2.1] - 2022-10-11
### Fixed
- get_param on wasm crashing if the URL had no parameters
- right mouse button context menu was not disabled on web platform

## [0.2.0] - 2022-10-10
### Changed
- upgrade to rust edition 2021
- upgrade to glutin 0.29.1
- replace unmaintained cargo-web/stdweb with wasm_bindgen/web-sys

## [0.1.4] - 2020-03-20
### Added
- support for DroppedFile events via App::get_dropped_files()
- upgrade to glutin 0.23

## [0.1.3] - 2019-12-04
### Added
- App::get_screen_resolution()

## [0.1.2] - 2019-11-29
### Added
- support for text input via winit's ReceivedCharacter event

## [0.1.1] - 2019-11-21
### Changed
- upgrade to stdweb 0.4.14
- upgrade to glutin 0.17
- now compiles on stable rust

### Added
- App::exit()

### Fixed
- arrow and pageup/pagedown keys on linux

## [0.1.0] - 2018-06-24
- initial release