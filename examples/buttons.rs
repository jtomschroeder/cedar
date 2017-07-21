
extern crate cedar;

use cedar::dom;
use cedar::dom::Builder;

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

fn view(model: &Model) -> dom::Object<Message> {
    dom::stack()
        .add(dom::button().text("+".into()).click(Message::Increment))
        .add(dom::label().text(model.to_string()))
        .add(dom::button().text("-".into()).click(Message::Decrement))
}

fn main() {
    cedar::program(0, update, view)
}