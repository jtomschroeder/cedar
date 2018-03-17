use std;
use std::hash::Hash;
use std::collections::HashSet;
use json;

use dom;
use shadow::Shadow;
use renderer;
use browser;
use processor;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

struct Program<M, S, R> where R: Eq + Hash {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    shadow: Shadow<S>,

    subscriber: Option<Subscriber<M, R>>,
    subscriptions: HashSet<R>,
}

impl<M, S, R> Program<M, S, R>
    where
        S: Send + PartialEq + 'static,
        M: Send + 'static,
        R: Send + Subscription<S> + 'static,
{
    fn new(model: M, update: Update<M, S>, view: View<M, S>, subscriber: Option<Subscriber<M, R>>) -> Self {
        let (shadow, commands) = Shadow::initialize(&model, view);

        Self::send(commands);

        let subscription = subscriber.map(|s| {
            let sub = s(&model);
            sub.enable();
            sub
        });

        let mut subscriptions= HashSet::new();
        if let Some(subscriber) = subscriber {
            let sub = subscriber(&model);
            sub.enable();
            subscriptions.insert(sub);
        }

        Program {
            model: Some(model),
            update,
            view,
            shadow,

            subscriber,
            subscriptions,
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
            let message = match self.shadow.translate(event, self.subscriptions.iter().next()) {
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

impl<M, S, R> processor::Processor for Program<M, S, R>
    where
        S: Send + PartialEq + 'static,
        M: Send + 'static,
        R: Send + Subscription<S> + 'static,
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
//    let program = Program::new(model, update, view, None);
//    processor::initialize(program);
}

pub fn programv<S, M, R>(
    (model, update, view, subscriber): (M, Update<M, S>, View<M, S>, Subscriber<M, R>),
) where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
    R: Send + Subscription<S> + 'static,
{
    browser::log("programv!");

    let program = Program::new(model, update, view, Some(subscriber));
    processor::initialize(program);
}

// fn program<S, M>(p: (M, Update<M, S>, View<M, S>));
// fn program<S, M>(p: (M, UpdateWithCmd<M, S>, View<M, S>, Subscriptions));
