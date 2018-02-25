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

// Use global as workaround for https://github.com/rust-lang/rust/issues/46489
static mut MODEL: Model = 0;

fn view(model: &Model) -> cedar::dom::Object<Message> {
    unsafe {
        MODEL = *model;

        hypertext! {
            <div>
                <button click={Message::Increment}> + </button>
                <div>{MODEL}</div>
                <button click={Message::Decrement}> - </button>
            </div>
        }
    }
}

fn main() {
    cedar::program(0, update, view)
}
