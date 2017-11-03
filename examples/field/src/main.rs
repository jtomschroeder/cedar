
#[macro_use]
extern crate cedar;

use cedar::dom::*;

type Model = String;

#[derive(PartialEq, Clone)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: Message) -> Model {
    match message {
        Message::NewContent(content) => content,
    }
}

fn style() -> Attribute {
    style!
    ( ("width", "100%")
    , ("height", "40px")
    , ("padding", "10px 0")
    , ("font-size", "2em")
    , ("text-align", "center")
    )
}

fn view(model: &Model) -> Object<Message> {
    div!(
        [],
        [
            input!([placeholder("Text to reverse!"), style()], []).change(Message::NewContent),
            div!([style()], [text(model.chars().rev().collect::<String>())]),
        ]
    )
}

fn main() {
    cedar::program("".into(), update, view)
}
