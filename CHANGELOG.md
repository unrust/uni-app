# Changelog

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