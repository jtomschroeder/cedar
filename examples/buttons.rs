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
    hypertext! {
        <div>
            <button click={Message::Increment}> + </button>
            <div>{model.to_string()}</div>
            <button click={Message::Decrement}> - </button>
        </div>
    }
}

fn main() {
    cedar::app(0, update, view)
}
