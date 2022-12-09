mod web_keycode;

use crate::AppConfig;
use js_sys::Uint8Array;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_keycode::*;
use web_sys::Event;
use web_sys::FileReader;
use web_sys::HtmlCanvasElement;

// use stdweb::traits::{IDragEvent, IEvent};
// use stdweb::web::event::{
//     DragDropEvent, IKeyboardEvent, IMouseEvent, KeyDownEvent, KeyUpEvent, MouseButton,
//     MouseDownEvent, MouseMoveEvent, MouseUpEvent, ResizeEvent,
// };
// use stdweb::web::html_element::CanvasElement;
// use stdweb::web::{window, IEventTarget, IHtmlElement, TypedArray};

use std::cell::RefCell;
use std::rc::Rc;

use crate::AppEvent;
use crate::{BufferState, File};

#[derive(Clone)]
pub struct App {
    app_canvas: HtmlCanvasElement,
    pub events: Rc<RefCell<Vec<AppEvent>>>,
    device_pixel_ratio: f32,
    dropped_files: Rc<RefCell<Vec<File>>>,
}

use super::events;

// In browser request full screen can only called under event handler.
// So basically this function is useless at this moment.
#[allow(dead_code)]
fn request_full_screen(canvas: &HtmlCanvasElement) {
    canvas.request_fullscreen().unwrap();
}

impl App {
    pub fn new(config: AppConfig) -> App {
        if config.headless {
            // Right now we did not support headless in web.
            unimplemented!();
        }
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas_element = document.create_element("canvas").unwrap();
        let app_canvas: HtmlCanvasElement = canvas_element.dyn_into::<HtmlCanvasElement>().unwrap();
        app_canvas.set_id("canvas");

        let device_pixel_ratio = window.device_pixel_ratio();
        // setup the buffer size
        // see https://webglfundamentals.org/webgl/lessons/webgl-resizing-the-canvas.html
        app_canvas.set_width((config.size.0 as f64 * device_pixel_ratio) as u32);
        app_canvas.set_height((config.size.1 as f64 * device_pixel_ratio) as u32);
        // setup the canvas size
        app_canvas
            .style()
            .set_property("width", &format!("{}px", config.size.0))
            .unwrap();
        app_canvas
            .style()
            .set_property("height", &format!("{}px", config.size.1))
            .unwrap();
        // Make it focusable
        // https://stackoverflow.com/questions/12886286/addeventlistener-for-keydown-on-canvas
        app_canvas.set_tab_index(1);

        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| event.prevent_default());
        document
            .add_event_listener_with_callback("dragover", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("dragenter", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("drop", closure.as_ref().unchecked_ref())
            .unwrap();
        app_canvas
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        if !config.show_cursor {
            app_canvas.style().set_property("cursor", "none").unwrap();
        }

        let body = document.body().unwrap();

        body.append_child(&app_canvas).unwrap();
        app_canvas.focus().unwrap();

        if config.fullscreen {
            println!("fullscreen not supported on webgl yet.");
        }

        let mut app = App {
            app_canvas,
            events: Rc::new(RefCell::new(Vec::new())),
            device_pixel_ratio: device_pixel_ratio as f32,
            dropped_files: Rc::new(RefCell::new(Vec::new())),
        };
        app.setup_listener();

        app
    }

    fn setup_listener(&mut self) {
        let events = self.events.clone();
        let mouse_down_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                events
                    .borrow_mut()
                    .push(AppEvent::MouseDown(events::MouseButtonEvent {
                        button: event.button() as usize,
                    }));
            });
        self.app_canvas
            .add_event_listener_with_callback(
                "mousedown",
                mouse_down_listener.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_down_listener.forget();
        let events = self.events.clone();
        let mouse_up_listener = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            events
                .borrow_mut()
                .push(AppEvent::MouseUp(events::MouseButtonEvent {
                    button: event.button() as usize,
                }));
        });
        self.app_canvas
            .add_event_listener_with_callback("mouseup", mouse_up_listener.as_ref().unchecked_ref())
            .unwrap();
        mouse_up_listener.forget();
        let events = self.events.clone();
        let move_canvas = self.app_canvas.clone();
        let mouse_move_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let canvas_rect = move_canvas.get_bounding_client_rect();
                let canvas_left = canvas_rect.left();
                let canvas_top = canvas_rect.top();
                event.prevent_default();
                events.borrow_mut().push(AppEvent::MousePos((
                    event.client_x() as f64 - canvas_left,
                    event.client_y() as f64 - canvas_top,
                )));
            });
        self.app_canvas
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_listener.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_move_listener.forget();
        let events = self.events.clone();
        let key_down_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                event.prevent_default();
                events
                    .borrow_mut()
                    .push(AppEvent::KeyDown(events::KeyDownEvent {
                        code: event.code(),
                        key: event.key(),
                        key_code: get_virtual_key(event).unwrap_or(VirtualKeyCode::Unlabeled),
                        shift: event.shift_key(),
                        alt: event.alt_key(),
                        ctrl: event.ctrl_key(),
                    }));
            });
        self.app_canvas
            .add_event_listener_with_callback("keydown", key_down_listener.as_ref().unchecked_ref())
            .unwrap();
        key_down_listener.forget();
        let events = self.events.clone();
        let key_up_listener = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            // filter control keys "Tab", "Backspace", ... for text input
            if event.key().len() == 1 {
                events
                    .borrow_mut()
                    .push(AppEvent::CharEvent(event.key().chars().next().unwrap()));
            }
            events
                .borrow_mut()
                .push(AppEvent::KeyUp(events::KeyUpEvent {
                    code: event.code(),
                    key: event.key(),
                    shift: event.shift_key(),
                    alt: event.alt_key(),
                    ctrl: event.ctrl_key(),
                }));
        });
        self.app_canvas
            .add_event_listener_with_callback("keyup", key_up_listener.as_ref().unchecked_ref())
            .unwrap();
        key_up_listener.forget();
        let events = self.events.clone();
        let resize_canvas = self.app_canvas.clone();
        let resize_listener = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            let width = resize_canvas.offset_width() as u32;
            let height = resize_canvas.offset_height() as u32;
            events.borrow_mut().push(AppEvent::Resized((width, height)));
        });
        window().set_onresize(Some(resize_listener.as_ref().unchecked_ref()));
        resize_listener.forget();
        let events = self.events.clone();
        let dropped_files = self.dropped_files.clone();
        let drag_drop_listener = Closure::<dyn FnMut(_)>::new(move |event: web_sys::DragEvent| {
            event.prevent_default();
            if let Some(data) = event.data_transfer() {
                if let Some(files) = data.files() {
                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            let buffer_state = Rc::new(RefCell::new(BufferState::Empty));
                            let name = file.name();
                            let reader = FileReader::new().unwrap();
                            let err_buffer_state = buffer_state.clone();
                            reader.set_onerror(Some(
                                Closure::<dyn FnMut(_)>::new(move |err: String| {
                                    let msg =
                                        format!("Fail to read file {} from web {}", name, err);
                                    *err_buffer_state.borrow_mut() = BufferState::Error(msg);
                                })
                                .as_ref()
                                .unchecked_ref(),
                            ));
                            let load_buffer_state = buffer_state.clone();
                            reader.set_onload(Some(
                                Closure::<dyn FnMut(_)>::new(
                                    move |event: web_sys::ProgressEvent| {
                                        let data =
                                            event.target().unwrap().into_js_result().unwrap();
                                        let array = Uint8Array::new(&data);
                                        *load_buffer_state.borrow_mut() =
                                            BufferState::Buffer(array.to_vec());
                                    },
                                )
                                .as_ref()
                                .unchecked_ref(),
                            ));
                            events
                                .borrow_mut()
                                .push(AppEvent::FileDropped(file.name().to_owned()));
                            dropped_files.borrow_mut().push(File {
                                buffer_state: buffer_state,
                            });
                        }
                    }
                }
            }
        });
        self.app_canvas
            .add_event_listener_with_callback("drop", drag_drop_listener.as_ref().unchecked_ref())
            .unwrap();
        drag_drop_listener.forget();
    }

    pub fn get_dropped_file(&mut self) -> Option<File> {
        self.dropped_files.borrow_mut().pop()
    }

    pub fn print<T: Into<String>>(msg: T) {
        let msg: String = msg.into();
        web_sys::console::log_1(&msg.into());
    }

    pub fn exit() {}

    pub fn get_screen_resolution(&self) -> (u32, u32) {
        (
            window().inner_width().unwrap().as_f64().unwrap() as u32,
            window().inner_height().unwrap().as_f64().unwrap() as u32,
        )
    }

    pub fn get_params() -> Vec<String> {
        let location = window().location().search().unwrap();
        if location.is_empty() {
            Vec::new()
        } else {
            // remove the leading ?
            let params = &location[1..];
            // return vector of "name=value" strings
            params
                .split("&")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        }
    }

    pub fn hidpi_factor(&self) -> f32 {
        return self.device_pixel_ratio;
    }

    pub fn run<F>(self, mut callback: F)
    where
        F: 'static + FnMut(&mut Self) -> (),
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut app = self.clone();
        *g.borrow_mut() = Some(Closure::new(move || {
            callback(&mut app);
            app.events.borrow_mut().clear();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.app_canvas
    }

    pub fn set_fullscreen(&mut self, _b: bool) {
        // unimplemented!();
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn now() -> f64 {
    // perforamce now is in ms
    // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    window().performance().unwrap().now() / 1000.0
}
