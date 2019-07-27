//#![feature(proc_macro_hygiene)]

use cedar::hypertext;
use cedar::sml;

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

type Object = cedar::dom::Object<Message>;

fn words(line: &str) -> Vec<Object> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|w| sml! { (div {w}) })
        .collect()
}

// TODO: input => onChange
// TODO: click => onClick

fn view(model: &Model) -> Object {
    sml! {
        (div (@ (class "tc"))
            (input (@ (class "mv3") (placeholder "Words!") (input Message::NewContent)))
            (div { words(model) })
        )
    }
}

fn main() {
    cedar::app("".into(), update, view)
}
