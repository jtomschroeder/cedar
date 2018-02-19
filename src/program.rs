use std::sync::mpsc::{channel, Receiver, Sender};
use serde_json as json;

use facade;
use dom;
use phantom::Phantom;
use renderer::{self, Renderer};
use browser;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub enum Action<S> {
    Update(S),
}

struct Program<M, S> {
    model: M,
    update: Update<M, S>,
    view: View<M, S>,
    phantom: Phantom<S>,
}

impl<M, S> Program<M, S>
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    pub fn new(model: M, update: Update<M, S>, view: View<M, S>) -> Self {
        let (phantom, commands) = Phantom::initialize(&model, view);

        for cmd in commands.into_iter() {
            browser::log(&format!("Event: {:?}", cmd));

            let cmd = json::to_string(&cmd).unwrap();
            browser::command(&cmd);
        }

        Program {
            model,
            update,
            view,
            phantom,
        }
    }
}

trait Processor {
    fn process(&mut self, s: String);
}

impl<M, S> Processor for Program<M, S>
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    fn process(&mut self, s: String) {
        browser::log(&format!("Processor!!: {}", s));

        //        let event = renderer.recv(); // blocking!
        //
        //        // translate events from backend renderer to actions
        //        let action = phantom.translate(event);
        //
        //        // TODO: `translate` could return (Action?, Commands?) to decouple layout from message
        //
        //        let action = match action {
        //            Some(a) => a,
        //            _ => continue,
        //        };
        //
        //        let commands = match action {
        //            Action::Update(message) => {
        //                model = update(model, message);
        //
        //                // TODO: might be better to change Update to fn(Model, &Message)
        //                // TODO: inject middleware here: middleware.handlers(&model, &message)
        //
        //                phantom.update(&model, view)
        //            }
        //        };
        //
        //        for event in commands.into_iter() {
        //            renderer.send(event);
        //        }

        let event = renderer::Event::Click {
            id: String::from("click-id"),
        };

        let action = self.phantom.translate(event);
    }
}

static mut PROCESSOR: Option<Box<Processor>> = None;

use std::ffi::CString;

#[no_mangle]
pub extern "C" fn process(x: i32, s: *mut i8) {
    browser::log(&format!("process: {:?}", x));

    unsafe {
        let s = CString::from_raw(s);
        let s = s.into_string().unwrap();

        if let Some(ref mut processor) = PROCESSOR {
            processor.process(s);
        }
    }
}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    browser::log("Hello, world!");

    let program = Program::new(model, update, view);
    unsafe { PROCESSOR = Some(Box::new(program)) };
}
