extern crate uni_app;
use std::process;

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
            uni_app::App::print(format!("{:?}\n",evt));
            // exit on key ou mouse press
            match evt {
                &uni_app::AppEvent::KeyUp(_) => {
                    process::exit(0);
                }
                &uni_app::AppEvent::MouseUp(_) => {
                    process::exit(0);
                }
                _ => (),
            }
        }
    });
}
