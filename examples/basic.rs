extern crate uni_app;
use uni_app::{App, AppConfig, AppEvent, FileSystem, ScanCode};

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
    let app = App::new(AppConfig {
        size: (800, 600),
        title: "my game".to_owned(),
        vsync: true,
        show_cursor: true,
        headless: false,
        resizable: true,
        fullscreen: false,
        intercept_close_request: false,
    });
    // loading a file
    let mut file = Some(FileSystem::open("test.txt").expect("Could not open the file test.txt.\nIn native mode, run this example in the www/ directory :\ncd www && cargo run --example basic"));
    // start game loop
    App::print("Press Escape to exit");
    app.run(move |app: &mut uni_app::App| {
        if let Some(the_file) = &mut file {
            if the_file.is_ready() {
                // when the file is ready, display its content
                let content = the_file
                    .read_text()
                    .expect("Could not read test.txt file's content");
                App::print(format!("File content : {}", content));
                // to avoid doing it every frame
                file = None;
            }
        }
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            App::print(format!("{:?}", evt));
            // exit when pressing escape
            match &evt {
                AppEvent::KeyUp(ev) if ev.code == ScanCode::Escape => {
                    App::exit();
                }
                _ => (),
            }
        }
    });
}
