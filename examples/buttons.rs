#![feature(proc_macro)]
#![feature(trace_macros)]

extern crate cedar;

use cedar::dom::*;
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

// <div>
//   <button click={Message::Increment}>+</button>
//   <div>{model}</div>
//   <button click={Message::Decrement}>-</button>
// </div>

fn view(model: &Model) -> cedar::dom::Object<Message> {
    // div().children(vec![
    //     button().add(text("+")).click(Message::Increment),
    //     div().add(text(model)),
    //     button().add(text("-")).click(Message::Decrement),
    // ])

    // trace_macros!(true);

    hypertext!(model)(model)
}

fn main() {
    cedar::program(0, update, view)
}
