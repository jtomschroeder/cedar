extern crate cedar;

use cedar::dom::*;

type Model = i32;

#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: &Message) -> Model {
    match message {
        &Message::Increment => model + 1,
        &Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> Object<Message> {
    div().children(vec![
        button().add(text("+")).click(Message::Increment),
        div().add(text(model)),
        button().add(text("-")).click(Message::Decrement),
    ])
}

fn main() {
    cedar::program(0, update, view)
}
