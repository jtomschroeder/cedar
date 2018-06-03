use json;
use std;
use std::collections::HashSet;
use std::hash::Hash;

use browser;
use dom;
use processor;
use renderer;
use shadow::Shadow;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

struct Program<M, S> {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    shadow: Shadow<S>,
}

impl<M, S> Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn new(model: M, update: Update<M, S>, view: View<M, S>) -> Self {
        let (shadow, commands) = Shadow::initialize(&model, view);

        Self::send(commands);

        Program {
            model: Some(model),
            update,
            view,
            shadow,
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

        // TODO: get new subscriptions
        // - Do a 'difference' on the old and new
        // - Enable new ones and disable old ones

        let model = {
            // translate events from backend renderer to actions
            let message = match self.shadow.translate(event) {
                Some(m) => m,
                _ => return,
            };

            let model = self.model.take().unwrap();
            (self.update)(model, &message)
        };

        let commands = {
            let commands = self.shadow.update(&model, self.view);
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
        Program::process(self, event)
    }
}

pub trait Subscription<S>: Eq + Hash {
    fn enable(&self);
    fn disable(&self);

    fn process(&self, value: json::Value) -> S;
}

pub type Subscriber<M, R> = fn(&M) -> R;

// Time.every : Time -> (Time -> msg) -> Sub msg
// e.g. Time.every second Tick

//impl Subscription for () {}

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    let program = Program::new(model, update, view);
    processor::initialize(program);
}

//pub fn programv<S, M, R>(
//    (model, update, view, subscriber): (M, Update<M, S>, View<M, S>, Subscriber<M, R>),
//) where
//    S: Send + PartialEq + 'static,
//    M: Send + 'static,
//    R: Send + Subscription<S> + 'static,
//{
//    browser::log("programv!");
//
//    let program = Program::new(model, update, view, Some(subscriber));
//    processor::initialize(program);
//}

// fn program<S, M>(p: (M, Update<M, S>, View<M, S>));
// fn program<S, M>(p: (M, UpdateWithCmd<M, S>, View<M, S>, Subscriptions));
