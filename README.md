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
uni-app = "0.3.*"
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
            uni_app::App::print(format!("{:?}", evt));
            // exit when pressing escape
            match &evt {
                uni_app::AppEvent::KeyUp(ev) if ev.code == ScanCode::Escape => {
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

Install wasm32 target :
```
rustup target install wasm32-unknown-unknown
```
Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
and [npm](https://www.npmjs.com/get-npm)

Compile the demo with
```
wasm-pack build examples
```
This creates a wasm package in examples/pkg

Run the demo with
```
cd www
npm install
npm run start
```

Open your browser at http://localhost:8080/

### As desktop app (native-opengl)

Run it from the www/ directory to be able to load the test.txt file :

```
cd www && cargo run --example basic --release
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

You can contribute to this library through pull requests. If you do so, please update the CHANGELOG.md and CREDITS.md files. If you provide a new feature, consider adding an example as a tutorial/showcase.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
