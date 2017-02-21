
extern crate cedar;
extern crate crossbeam;

use std::sync::{Arc, Mutex};

use cedar::{Stream, View};

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

fn view(queue: Stream<Message>) -> View {
    View::new()
        .button(|button| {
            let queue = queue.clone();
            button.text("+")
                .position(50., 100.)
                .click(move || queue.push(Message::Increment))
            // .click(|| Message::Increment)
        })
        .button(|button| {
            let queue = queue.clone();
            button.text("-")
                .position(150., 100.)
                .click(move || queue.push(Message::Decrement))
            // .click(|| Message::Decrement)
        })
        .label(|label| {
            label.text(|model: Model| model.to_string())
                .position(100., 100.)
        })
}

fn main() {
    let queue = Stream::<Message>::new();

    let app = cedar::cacao::Application::new();

    let mut view = view(queue.clone());

    let mut model = 0;
    view.update(model);

    let view = Arc::new(Mutex::new(view));

    std::thread::spawn(move || loop {
        let message = queue.pop();
        model = update(model, message);

        if let Ok(mut view) = view.lock() {
            view.update(model);
        }
    });

    app.run()
}
