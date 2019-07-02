#![feature(proc_macro_hygiene)]

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

const STYLE: &'static str = "";
    // "width: 100%; height: 40px; padding: 10px 0; font-size: 2em; text-align: center;";

fn view(model: &Model) -> cedar::dom::Object<Message> {
    let field: String = model.chars().rev().collect();

    hypertext! {
        <div class={"tc f2"}>
            <input style={STYLE} class={"w-100 tc pa3"} input={Message::NewContent}></input>
            <div style={STYLE}>{field}</div>
        </div>
    }
}

fn main() {
    cedar::app("".into(), update, view)
}
