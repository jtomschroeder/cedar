#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate cedar;

use cedar::hypertext;

type Model = String;

#[derive(PartialEq)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: &Message) -> Model {
    match message {
        &Message::NewContent(ref content) => content.clone(),
    }
}

const STYLE: &'static str =
    "width: 100%; height: 40px; padding: 10px 0; font-size: 2em; text-align: center;";

fn view(model: &Model) -> cedar::dom::Object<Message> {
    let field: String = model.chars().rev().collect();

    (hypertext! { |field|
        <div>
            <input style={STYLE} input={Message::NewContent}></input>
            <div style={STYLE}>{field}</div>
        </div>
    })(field)
}

fn main() {
    cedar::program("".into(), update, view)
}
