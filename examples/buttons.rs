
extern crate cedar;
extern crate crossbeam;

use std::sync::{Arc, Mutex};
use crossbeam::sync::MsQueue;

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

fn view(queue: Arc<MsQueue<Message>>) -> cedar::cacao::Window {
    let mut window = cedar::cacao::Window::new("buttons");

    let inc = {
        let queue = queue.clone();
        cedar::cacao::Button::new()
            .text("+")
            .position(50., 100.)
            .click(move || queue.push(Message::Increment))
    };

    let dec = {
        let queue = queue.clone();
        cedar::cacao::Button::new()
            .text("-")
            .position(150., 100.)
            .click(move || queue.push(Message::Decrement))
    };

    let label = cedar::cacao::Label::new()
        .text(|model: Model| model.to_string())
        .position(100., 100.);

    window.add(inc);
    window.add(dec);
    window.add(label);

    window
}

fn main() {
    let queue = Arc::new(MsQueue::<Message>::new());

    let app = cedar::cacao::Application::new();

    let mut window = view(queue.clone());

    let mut model = 0;
    window.update(model);

    let window = Arc::new(Mutex::new(window));

    std::thread::spawn(move || loop {
        let message = queue.pop();
        model = update(model, message);

        if let Ok(mut window) = window.lock() {
            window.update(model);
        }
    });

    app.run()
}
