use std;
use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;
use std::str;

use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType};

pub type IoError = std::io::Error;

pub struct FileSystem {}

pub(crate) enum BufferState {
    Empty,
    Buffer(Vec<u8>),
    Error(String),
}

pub struct File {
    pub(crate) buffer_state: Rc<RefCell<BufferState>>,
}

impl FileSystem {
    pub fn open(file_url: &str) -> Result<File, IoError> {
        let buffer_state = Rc::new(RefCell::new(BufferState::Empty));

        let ref_req = Rc::new(RefCell::new(XmlHttpRequest::new().unwrap()));
        let req = &ref_req.borrow();
        req.open("GET", file_url).unwrap();
        req.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
        let load_req = ref_req.clone();
        let load_buffer_state = buffer_state.clone();
        let load_listener = Closure::<dyn FnMut(_)>::new(move |_: web_sys::ProgressEvent| {
            let req = load_req.borrow();
            let status = req.status().unwrap();
            if status == 200 {
                if let Ok(data) = req.response() {
                    let array = Uint8Array::new(&data);
                    *load_buffer_state.borrow_mut() = BufferState::Buffer(array.to_vec());
                    return;
                }
            }
            *load_buffer_state.borrow_mut() =
                BufferState::Error("Fail to read file from web".to_string());
        });
        req.set_onload(Some(load_listener.as_ref().unchecked_ref()));
        load_listener.forget();
        let err_buffer_state = buffer_state.clone();
        let err_listener = Closure::<dyn FnMut(_)>::new(move |_: web_sys::ProgressEvent| {
            *err_buffer_state.borrow_mut() =
                BufferState::Error("Fail to read file from web".to_string());
        });
        req.set_onerror(Some(err_listener.as_ref().unchecked_ref()));
        err_listener.forget();
        req.send().unwrap();

        Ok(File {
            buffer_state: buffer_state,
        })
    }
}

impl File {
    pub fn is_ready(&self) -> bool {
        let bs = self.buffer_state.borrow();
        match *bs {
            BufferState::Empty => false,
            BufferState::Error(_) => true,
            BufferState::Buffer(_) => true,
        }
    }

    pub fn read_binary(&mut self) -> Result<Vec<u8>, IoError> {
        let mut bs = self.buffer_state.borrow_mut();
        match *bs {
            BufferState::Error(ref s) => Err(std::io::Error::new(ErrorKind::Other, s.clone())),
            BufferState::Buffer(ref mut v) => Ok({
                let mut r = Vec::new();
                r.append(v);
                r
            }),
            _ => unreachable!(),
        }
    }

    pub fn read_text(&mut self) -> Result<String, IoError> {
        let mut bs = self.buffer_state.borrow_mut();
        match *bs {
            BufferState::Error(ref s) => Err(std::io::Error::new(ErrorKind::Other, s.clone())),
            BufferState::Buffer(ref mut v) => match str::from_utf8(v) {
                Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
                Ok(v) => Ok(v.to_owned()),
            },
            _ => unreachable!(),
        }
    }
}
