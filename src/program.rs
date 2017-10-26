
use std::thread;

// use facade;
use dom;
use phantom::Phantom;
use renderer::Renderer;

mod facade {
    use renderer::{self, Command, Event};

    extern "C" {
        fn cef_app_run();
    }

    #[derive(Clone)]
    pub struct Renderer {}

    impl Renderer {
        pub fn new() -> Self {
            Renderer {}
        }
    }

    impl renderer::Renderer for Renderer {
        fn send(&self, cmd: Command) {}

        fn recv(&self) -> Event {
            unimplemented!()
        }
    }

    pub fn run(_: Renderer) {
        unsafe { cef_app_run() }
    }

    mod ffi {
        // from https://github.com/jtomschroeder/cedar/blob/fa1345b9c35245f25546da5853073daa4dca7ec2/src/cocoa/mod.rs

        use std::os::raw::{c_void, c_char};
        use std::ffi::{CStr, CString};
        use std::sync::Arc;

        use crossbeam::sync::MsQueue;
        use serde_json as json;

        use renderer::{self, Command, Event};

        mod bindings {
            use super::*;
            extern "C" {
                pub fn run(renderer: *mut c_void);
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
            unsafe { bindings::run(Box::into_raw(renderer) as *mut c_void) }
        }
    }
}

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub enum Action<S> {
    Update(S),
}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    println!("Hello, world!");

    let renderer = facade::Renderer::new();

    //
    // TODO: separate `model` and `update` from `view` and `renderer`
    // - `model` & `update` => fluxion (flux)
    // - `view` & `renderer` => phantom (i.e. shadow DOM)
    //
    // - use single (blocking) queue for events from backend & events from 'effects'
    //   - effects => commands, subscriptions
    //

    {
        let renderer = renderer.clone();

        thread::spawn(move || {
            let (mut phantom, commands) = Phantom::initialize(&model, view);

            for event in commands.into_iter() {
                renderer.send(event);
            }

            // Receive messages from 'renderer'

            loop {
                let event = renderer.recv(); // blocking!

                // translate events from backend renderer to actions
                let action = phantom.translate(event);

                // TODO: `translate` could return (Action?, Commands?) to decouple layout from message

                let action = match action {
                    Some(a) => a,
                    _ => continue,
                };

                let commands = match action {
                    Action::Update(message) => {
                        model = update(model, message);

                        // TODO: might be better to change Update to fn(Model, &Message)
                        // TODO: inject middleware here: middleware.handlers(&model, &message)

                        phantom.update(&model, view)
                    }
                };

                for event in commands.into_iter() {
                    renderer.send(event);
                }
            }
        });
    }

    facade::run(renderer) // run renderer on 'main' thread!
}
