
extern crate cedar;

use cedar::{Application, View};

type Model = String;

enum Message {
    NewContent(String),
}

fn update(_: Model, message: Message) -> Model {
    match message {
        Message::NewContent(content) => content,
    }
}

fn view() -> View<Model, Message> {
    View::new()
        .field(|field| {
            field.placeholder("Text to reverse")
                .position(50., 200.)
                .change(|s| Message::NewContent(s.chars().rev().collect()))
        })
        .label(|label| {
            label.text(|model: Model| model)
                .position(100., 100.)
        })
}

fn main() {
    let app = Application::new("--".into(), update, view);
    app.run()
}
