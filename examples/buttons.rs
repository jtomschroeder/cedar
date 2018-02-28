#![feature(proc_macro)]

extern crate cedar;

use cedar::hypertext;

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

fn view(model: &Model) -> cedar::dom::Object<Message> {
    (hypertext! { |model|
        <div>
            <button click={Message::Increment}> + </button>
            <div>{model}</div>
            <button click={Message::Decrement}> - </button>
        </div>
    })(model)
}

fn main() {
    cedar::program(0, update, view)
}
