
extern crate cedar;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
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

use cedar::dom;

fn view(model: &Model) -> dom::Object<Message> {
    dom::stack(vec![
        dom::button("+".into()).click(Message::Increment),
        dom::label(model.to_string()),
        dom::button("-".into()).click(Message::Decrement),
    ])
}

fn main() {
    cedar::program(0, update, view)
}