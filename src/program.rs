use serde_json as json;

use dom;
use shadow::Shadow;
use renderer;
use browser;
use processor;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

// pub trait TProgram<M, S> {
//     fn init() -> M {}
//     fn update(M, &S) -> M {}
//     fn view() {}
//     fn subscriptions() {}
// }

struct Program<M, S> {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    phantom: Shadow<S>,
}

impl<M, S> Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn new(model: M, update: Update<M, S>, view: View<M, S>) -> Self {
        let (phantom, commands) = Shadow::initialize(&model, view);

        Self::send(commands);

        Program {
            model: Some(model),
            update,
            view,
            phantom,
        }
    }

    fn send(commands: Vec<renderer::Command>) {
        for cmd in commands.into_iter() {
            let cmd = json::to_string(&cmd).unwrap();
            browser::command(&cmd);
        }
    }

    fn process(&mut self, event: String) {
        let event: renderer::Event = json::from_str(&event).unwrap();

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

        Self::send(commands);
    }
}

impl<M, S> processor::Processor for Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn process(&mut self, event: String) {
        self.process(event)
    }
}

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    let program = Program::new(model, update, view);
    processor::initialize(program);
}

// fn program<S, M>(p: (M, Update<M, S>, View<M, S>));
// fn program<S, M>(p: (M, UpdateWithCmd<M, S>, View<M, S>, Subscriptions));
