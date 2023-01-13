mod native_keycode;

use glutin;
use glutin::dpi::LogicalSize;
use glutin::event::ElementState;
use glutin::event::Event;
use glutin::event::KeyboardInput;
use glutin::event::ModifiersState;
use glutin::event::MouseButton;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::EventLoop;
use glutin::monitor::VideoMode;
use glutin::window::Fullscreen;
use glutin::window::WindowBuilder;
use glutin::Context;
use glutin::PossiblyCurrent;
use glutin::WindowedContext;
use std::cell::RefCell;
use std::env;
use std::os::raw::c_void;
use std::process;
use std::rc::Rc;
use time;

use crate::AppConfig;
use crate::AppEvent;

use self::native_keycode::{translate_scan_code, translate_virtual_key};
use crate::events;
use crate::{File, FileSystem};

enum WindowContext {
    Headless(Context<PossiblyCurrent>),
    Normal(WindowedContext<PossiblyCurrent>),
}

impl WindowContext {
    fn hidpi_factor(&self) -> f32 {
        match self {
            WindowContext::Normal(ref w) => w.window().scale_factor() as f32,
            _ => 1.0,
        }
    }

    fn window(&self) -> &WindowedContext<PossiblyCurrent> {
        match self {
            WindowContext::Normal(ref w) => w,
            _ => unimplemented!(),
        }
    }

    fn context(&self) -> &Context<PossiblyCurrent> {
        match self {
            WindowContext::Normal(w) => w.context(),
            WindowContext::Headless(w) => w,
        }
    }

    fn swap_buffers(&self) -> Result<(), glutin::ContextError> {
        match self {
            WindowContext::Normal(ref w) => w.swap_buffers(),
            WindowContext::Headless(_) => Ok(()),
        }
    }
}

/// the main application struct
pub struct App {
    window: WindowContext,
    events_loop: Option<EventLoop<()>>,
    intercept_close_request: bool,
    modifiers_state: ModifiersState,
    pub events: Rc<RefCell<Vec<AppEvent>>>,
    dropped_files: Vec<File>,
    fullscreen_resolution: VideoMode,
}

fn get_virtual_key(input: KeyboardInput) -> String {
    match input.virtual_keycode {
        Some(k) => {
            let mut s = translate_virtual_key(k).into();
            if s == "" {
                s = format!("{:?}", k);
            }
            s
        }
        None => "".into(),
    }
}

fn get_scan_code(input: KeyboardInput) -> events::ScanCode {
    translate_scan_code(input.scancode & 0xFF)
}

fn translate_event(e: Event<()>, modifiers: &ModifiersState) -> Option<AppEvent> {
    if let Event::WindowEvent {
        event: winevent, ..
    } = e
    {
        match winevent {
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    MouseButton::Left => events::MouseButton::Left,
                    MouseButton::Middle => events::MouseButton::Middle,
                    MouseButton::Right => events::MouseButton::Right,
                    MouseButton::Other(val) => events::MouseButton::Other(val as usize),
                };
                let event = events::MouseButtonEvent { button: mouse_button };
                match state {
                    ElementState::Pressed => Some(AppEvent::MouseDown(event)),
                    ElementState::Released => Some(AppEvent::MouseUp(event)),
                }
            }
            WindowEvent::CursorMoved { position, .. } => Some(AppEvent::MousePos(position.into())),
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => Some(AppEvent::KeyDown(events::KeyDownEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    shift: modifiers.shift(),
                    alt: modifiers.alt(),
                    ctrl: modifiers.ctrl(),
                })),
                ElementState::Released => Some(AppEvent::KeyUp(events::KeyUpEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    shift: modifiers.shift(),
                    alt: modifiers.alt(),
                    ctrl: modifiers.ctrl(),
                })),
            },
            WindowEvent::ReceivedCharacter(c) => Some(AppEvent::CharEvent(c)),
            WindowEvent::Resized(size) => Some(AppEvent::Resized(size.into())),
            WindowEvent::CloseRequested => Some(AppEvent::CloseRequested),
            WindowEvent::DroppedFile(path) => {
                Some(AppEvent::FileDropped(path.to_str().unwrap().to_owned()))
            }
            _ => None,
        }
    } else {
        None
    }
}

impl App {
    /// create a new game window
    pub fn new(config: AppConfig) -> App {
        use glutin::*;
        let events_loop = EventLoop::new();
        let gl_req = GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (2, 0),
        };
        let fullscreen_resolution = events_loop
            .available_monitors()
            .nth(0)
            .unwrap()
            .video_modes()
            .nth(0)
            .unwrap();
        let window = if config.headless {
            let headless_context = ContextBuilder::new()
                .with_gl(gl_req)
                .with_gl_profile(GlProfile::Core)
                .build_headless(&events_loop, (config.size.0, config.size.1).into())
                .unwrap();

            WindowContext::Headless(unsafe { headless_context.make_current().unwrap() })
        } else {
            let window_builder = WindowBuilder::new()
                .with_title(config.title)
                .with_fullscreen(if config.fullscreen {
                    Some(Fullscreen::Exclusive(fullscreen_resolution.clone()))
                } else {
                    None
                })
                .with_resizable(config.resizable)
                .with_inner_size(LogicalSize::new(config.size.0, config.size.1));

            let windowed_context = ContextBuilder::new()
                .with_vsync(config.vsync)
                .with_gl(gl_req)
                .with_gl_profile(GlProfile::Core)
                .build_windowed(window_builder, &events_loop)
                .unwrap();

            windowed_context
                .window()
                .set_cursor_visible(config.show_cursor);

            WindowContext::Normal(unsafe { windowed_context.make_current().unwrap() })
        };

        App {
            window,
            events_loop: Some(events_loop),
            intercept_close_request: config.intercept_close_request,
            events: Rc::new(RefCell::new(Vec::new())),
            dropped_files: Vec::new(),
            modifiers_state: ModifiersState::default(),
            fullscreen_resolution,
        }
    }

    /// return the screen resolution in physical pixels
    pub fn get_screen_resolution(&self) -> (u32, u32) {
        if let WindowContext::Normal(ref glwindow) = self.window {
            if let Some(ref monitor) = glwindow.window().current_monitor() {
                return monitor.size().into();
            }
        }
        (0, 0)
    }

    /// return the command line / URL parameters
    pub fn get_params() -> Vec<String> {
        let mut params: Vec<String> = env::args().collect();
        params.remove(0);
        params
    }

    /// activate or deactivate fullscreen. only works on native target
    pub fn set_fullscreen(&mut self, b: bool) {
        if let WindowContext::Normal(ref glwindow) = self.window {
            if b {
                glwindow.window().set_fullscreen(Some(Fullscreen::Exclusive(
                    self.fullscreen_resolution.clone(),
                )));
            } else {
                glwindow.window().set_fullscreen(None);
            }
        }
    }

    /// print a message on standard output (native) or js console (web)
    pub fn print<T: Into<String>>(msg: T) {
        println!("{}", msg.into());
    }

    /// exit current process (close the game window). On web target, this does nothing.
    pub fn exit() {
        process::exit(0);
    }

    /// returns the HiDPI factor for current screen
    pub fn hidpi_factor(&self) -> f32 {
        self.window.hidpi_factor()
    }

    fn get_proc_address(&self, name: &str) -> *const c_void {
        self.window.context().get_proc_address(name) as *const c_void
    }

    /// return the opengl context for this window
    pub fn canvas<'p>(&'p self) -> Box<dyn 'p + FnMut(&str) -> *const c_void> {
        Box::new(move |name| self.get_proc_address(name))
    }

    fn handle_event(&mut self, event: Event<()>) -> (bool, bool) {
        let mut running = true;
        let mut next_frame = false;
        match event {
            Event::RedrawRequested(_) => {}
            Event::MainEventsCleared => {
                next_frame = true;
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if !self.intercept_close_request {
                        running = false;
                    }
                }
                WindowEvent::Resized(size) => {
                    // Fixed for Windows which minimized to emit a Resized(0,0) event
                    if size.width != 0 && size.height != 0 {
                        self.window.window().resize(*size);
                    }
                }
                WindowEvent::ModifiersChanged(new_state) => {
                    self.modifiers_state = *new_state;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    // issue tracked in https://github.com/tomaka/winit/issues/41
                    // Right now we handle it manually.
                    if cfg!(target_os = "macos") {
                        if let Some(keycode) = input.virtual_keycode {
                            if keycode == VirtualKeyCode::Q && self.modifiers_state.logo() {
                                running = false;
                            }
                        }
                    }
                }
                WindowEvent::DroppedFile(ref path) => {
                    let filepath = path.to_str().unwrap();
                    self.dropped_files.push(FileSystem::open(filepath).unwrap());
                }
                _ => (),
            },
            _ => (),
        };

        if let Some(app_event) = translate_event(event, &self.modifiers_state) {
            self.events.borrow_mut().push(app_event);
        }

        (running, next_frame)
    }

    pub fn get_dropped_file(&mut self) -> Option<File> {
        self.dropped_files.pop()
    }

    /// start the game loop, calling provided callback every frame
    pub fn run<'a, F>(mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut Self) -> (),
    {
        let events_loop = self.events_loop.take().unwrap();
        events_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            let (running, next_frame) = self.handle_event(event);
            if !running {
                control_flow.set_exit();
            }
            if next_frame {
                callback(&mut self);
                self.events.borrow_mut().clear();
                self.window.swap_buffers().unwrap();
            }
        });
    }
}

/// return the time since the start of the program in seconds
pub fn now() -> f64 {
    // precise_time_s() is in second
    // https://doc.rust-lang.org/time/time/fn.precise_time_s.html
    time::precise_time_s()
}
