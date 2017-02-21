
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

fn view() -> View<Message> {
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

trait Viewable<S> {
    fn view(&mut self) -> View<S>;
}

impl<S, F> Viewable<S> for F
    where F: FnMut() -> View<S>
{
    fn view(&mut self) -> View<S> {
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

struct Application<U, V> {
    update: U,
    view: V,
}

impl<U, V> Application<U, V> {
    pub fn new(update: U, view: V) -> Self {
        Application {
            update: update,
            view: view,
        }
    }
}

impl<U, V> Application<U, V>
    where U: Update<Model, Message> + Send + 'static,
          V: Viewable<Message>
{
    pub fn run(mut self) {
        let app = cedar::cacao::Application::new();

        let mut view = self.view.view();

        let mut model = 0;
        view.update(model);

        let view = Arc::new(Mutex::new(view));

        let mut update = self.update;
        std::thread::spawn(move || loop {
            if let Ok(mut view) = view.lock() {
                let message = view.stream().pop();
                model = update.update(model, message);
                view.update(model);
            }
        });

        app.run()
    }
}

////

fn main() {
    let app = Application::new(update, view);
    app.run()
}
