
extern crate crossbeam;
extern crate cedar;

use cedar::View;

type Model = i32;

#[derive(Debug)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: Message) -> Model {
    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

fn view() -> View<Model, Message> {
    View::new()
        .button(|button| {
            button.text("+")
                .position(50., 100.)
                .click(|| Message::Increment)
        })
        .button(|button| {
            button.text("-")
                .position(150., 100.)
                .click(|| Message::Decrement)
        })
        .label(|label| {
            label.text(|model: Model| model.to_string())
                .position(100., 100.)
        })
}

////

use std::sync::{Arc, Mutex};

trait Viewable<M, S> {
    fn view(&mut self) -> View<M, S>;
}

impl<M, S, F> Viewable<M, S> for F
    where F: FnMut() -> View<M, S>
{
    fn view(&mut self) -> View<M, S> {
        self()
    }
}

trait Update<M, S> {
    fn update(&mut self, model: M, message: S) -> M;
}

impl<M, S, F> Update<M, S> for F
    where F: FnMut(M, S) -> M
{
    fn update(&mut self, model: M, message: S) -> M {
        self(model, message)
    }
}

struct Application<S, M, U, V> {
    model: M,
    update: U,
    view: V,
    message: std::marker::PhantomData<S>,
}

impl<S, M, U, V> Application<S, M, U, V> {
    pub fn new(model: M, update: U, view: V) -> Self {
        Application {
            model: model,
            update: update,
            view: view,
            message: std::marker::PhantomData,
        }
    }
}

impl<S, M, U, V> Application<S, M, U, V>
    where S: Send + 'static,
          M: Clone + Send + 'static,
          U: Update<M, S> + Send + 'static,
          V: Viewable<M, S>
{
    pub fn run(mut self) {
        let app = cedar::cacao::Application::new();

        let mut view = self.view.view();

        let mut model = self.model;
        view.update(model.clone());

        let view = Arc::new(Mutex::new(view));

        let mut update = self.update;
        std::thread::spawn(move || loop {
            if let Ok(mut view) = view.lock() {
                let message = view.stream().pop();
                model = update.update(model.clone(), message);
                view.update(model.clone());
            }
        });

        app.run()
    }
}

////

fn main() {
    let app = Application::new(0, update, view);
    app.run()
}
