
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

struct Application<U> {
    update: U,
}

impl<U> Application<U> {
    pub fn new(update: U) -> Self {
        Application { update: update }
    }
}

impl<U> Application<U>
    where U: Update<Model, Message> + Send + 'static
{
    pub fn run(mut self) {
        let app = cedar::cacao::Application::new();

        let mut view = view();

        let mut model = 0;
        view.update(model);

        let view = Arc::new(Mutex::new(view));

        std::thread::spawn(move || loop {
            if let Ok(mut view) = view.lock() {
                let message = view.stream().pop();
                model = self.update.update(model, message);
                view.update(model);
            }
        });

        app.run()
    }
}

////

fn main() {
    let app = Application::new(update);
    app.run()
}
