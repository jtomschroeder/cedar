
// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_use]
extern crate cedar;

use cedar::dom::{Object, text};

type Model = i32;

#[derive(PartialEq, Clone)]
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

fn view(model: &Model) -> Object<Message> {
    // dom::div(vec![
    //     dom::button("+".into()).click(Message::Increment),
    //     dom::div(vec![dom::text(model.to_string())]),
    //     dom::button("-".into()).click(Message::Decrement),
    // ])

    // view!{};

    // view! {
    //     div => []
    // }

    // view! {
    //     div => [
    //         button => [],
    //     ]
    // }

    div!(
        [],
        [
            button!([], [text("+")]).click(Message::Increment),
            div!([], [text(model)]),
            button!([], [text("-")]).click(Message::Decrement),
        ]
    )
}

fn main() {
    cedar::program(0, update, view)
}
