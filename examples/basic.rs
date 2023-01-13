extern crate uni_app;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

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
        intercept_close_request: false,
    });
    // start game loop
    uni_app::App::print("Press Escape to exit");
    app.run(move |app: &mut uni_app::App| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            uni_app::App::print(format!("{:?}", evt));
            // exit on key or mouse press
            match &evt {
                uni_app::AppEvent::KeyUp(ev) if ev.code == "Escape" => {
                    uni_app::App::exit();
                }
                _ => (),
            }
        }
    });
}
