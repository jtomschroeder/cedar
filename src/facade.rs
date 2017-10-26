
// use renderer::{self, Command, Event};
// extern "C" {
//     fn cef_app_run();
// }
// #[derive(Clone)]
// pub struct Renderer {}
// impl Renderer {
//     pub fn new() -> Self {
//         Renderer {}
//     }
// }
// impl renderer::Renderer for Renderer {
//     fn send(&self, cmd: Command) {}
//     fn recv(&self) -> Event {
//         unimplemented!()
//     }
// }
// pub fn run(_: Renderer) {
//     unsafe { cef_app_run() }
// }

use std::os::raw::{c_void, c_char};
use std::ffi::{CStr, CString};
use std::sync::Arc;

use crossbeam::sync::MsQueue;
use serde_json as json;

use renderer::{self, Command, Event};

mod bindings {
    use super::*;
    extern "C" {
        pub fn cef_app_run(renderer: *mut c_void);
    }
}

#[derive(Clone)]
pub struct Renderer {
    commands: Arc<MsQueue<String>>,
    events: Arc<MsQueue<String>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            commands: Arc::new(MsQueue::new()),
            events: Arc::new(MsQueue::new()),
        }
    }
}

impl renderer::Renderer for Renderer {
    fn send(&self, cmd: Command) {
        let cmd = json::to_string(&cmd).unwrap();
        self.commands.push(cmd)
    }

    fn recv(&self) -> Event {
        loop {
            let line = self.events.pop();
            match json::from_str(&line) {
                Ok(event) => return event,
                Err(err) => {
                    eprintln!("Failed to parse event: '{}' :: {:?}", line, err);
                }
            }
        }
    }
}

// TODO: handling dropping of renderer instance

#[no_mangle]
pub extern "C" fn renderer_recv(renderer: *mut Renderer) -> *mut c_char {
    let renderer: &Renderer = unsafe { &*renderer };

    let input = renderer.commands.pop(); // blocking!

    let string = CString::new(input.into_bytes()).unwrap();
    CString::into_raw(string)
}

#[no_mangle]
pub extern "C" fn renderer_resp(renderer: *mut Renderer, s: *const c_char) {
    let renderer: &Renderer = unsafe { &*renderer };

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    renderer.events.push(s.into());
}

#[no_mangle]
pub extern "C" fn renderer_string_drop(s: *mut c_char) {
    let _ = unsafe { CString::from_raw(s) };
}

pub fn run(renderer: Renderer) {
    let renderer = Box::new(renderer);
    unsafe { bindings::cef_app_run(Box::into_raw(renderer) as *mut c_void) }
}
