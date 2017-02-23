
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
                .change(|s| Message::NewContent(s.chars().rev().collect()))
        })
        .label(|label| label.text(|model: Model| model))
}

fn main() {
    let app = Application::new("--".into(), update, view);
    app.run()
}
