use std::ffi::CString;
use serde_json as json;

use dom;
use phantom::Phantom;
use renderer;
use browser;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

fn send(commands: Vec<renderer::Command>) {
    for cmd in commands.into_iter() {
        let cmd = json::to_string(&cmd).unwrap();
        browser::command(&cmd);
    }
}

struct Program<M, S> {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    phantom: Phantom<S>,
}

impl<M, S> Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    pub fn new(model: M, update: Update<M, S>, view: View<M, S>) -> Self {
        let (phantom, commands) = Phantom::initialize(&model, view);

        send(commands);

        Program {
            model: Some(model),
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
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn process(&mut self, s: String) {
        let event: renderer::Event = json::from_str(&s).unwrap();

        let model = {
            // translate events from backend renderer to actions
            let message = match self.phantom.translate(event) {
                Some(m) => m,
                _ => return,
            };

            let model = self.model.take().unwrap();
            (self.update)(model, &message)
        };

        let commands = {
            // TODO: inject middleware here: middleware.handlers(&model, &message)

            let commands = self.phantom.update(&model, self.view);
            self.model = Some(model);
            commands
        };

        send(commands);
    }
}

static mut PROCESSOR: Option<Box<Processor>> = None;

#[no_mangle]
pub extern "C" fn process(s: *mut i8) {
    unsafe {
        let s = CString::from_raw(s);
        let s = s.into_string().unwrap();

        if let Some(ref mut processor) = PROCESSOR {
            processor.process(s);
        }
    }
}

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    let program = Program::new(model, update, view);
    unsafe { PROCESSOR = Some(Box::new(program)) };
}
