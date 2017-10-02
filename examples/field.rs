
extern crate cedar;

use cedar::dom;
use cedar::dom::Builder;

type Model = String;

#[derive(PartialEq, Clone, Debug)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: Message) -> Model {
    match message {
        Message::NewContent(content) => content,
    }
}

fn view(model: &Model) -> dom::Object<Message> {
    dom::stack(vec![
        dom::field()
            .placeholder("Text to reverse!".into())
            .change(Message::NewContent),
        dom::label().text(model.chars().rev().collect()),
    ])
}

fn main() {
    cedar::program("--".into(), update, view)
}
