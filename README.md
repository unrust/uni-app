# unrust / uni-app

[![Build Status](https://travis-ci.org/unrust/uni-app.svg?branch=master)](https://travis-ci.org/unrust/uni-app)
[![Documentation](https://docs.rs/uni-app/badge.svg)](https://docs.rs/uni-app)
[![crates.io](https://meritbadge.herokuapp.com/uni-app)](https://crates.io/crates/uni-app)

This library is a part of [Unrust](https://github.com/unrust/unrust), a pure rust native/wasm game engine.
This library provides a native/wasm compatibility layer for following components :
* Window creation
* Input (mouse + keyboard)
* File system (ready-only)

## Usage

```toml
[dependencies]
uni-app = "0.2.*"
```

```rust
extern crate uni_app;

fn main() {
    // create the game window (native) or canvas (web)
    let app = uni_app::App::new(uni_app::AppConfig {
        size: (800, 600),
        title: "my game".to_owned(),
        vsync: true,
        show_cursor: true,
        headless: false,
        resizable: true,
        fullscreen: false,
    });
    // start game loop
    app.run(move |app: &mut uni_app::App| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            uni_app::App::print(format!("{:?}\n", evt));
            // exit on key ou mouse press
            match evt {
                &uni_app::AppEvent::KeyUp(_) => {
                    uni_app::App::exit();
                }
                &uni_app::AppEvent::MouseUp(_) => {
                    uni_app::App::exit();
                }
                _ => (),
            }
        }
    });
}
```

## Build

### As web app (wasm32-unknown-unknown)

```
cargo install --force cargo-web # installs web sub command
rustup target install wasm32-unknown-unknown
cargo web start --example basic --release
```

### As desktop app (native-opengl)

```
cargo run --example basic --release
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
