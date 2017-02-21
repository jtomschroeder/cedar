
extern crate cedar;
extern crate crossbeam;

use std::sync::{Arc, Mutex};

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

fn main() {
    let app = cedar::cacao::Application::new();

    let mut view = view();

    let mut model = 0;
    view.update(model);

    let view = Arc::new(Mutex::new(view));

    std::thread::spawn(move || loop {
        if let Ok(mut view) = view.lock() {
            let message = view.stream().pop();
            model = update(model, message);
            view.update(model);
        }
    });

    app.run()
}
