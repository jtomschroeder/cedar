
#[macro_use]
extern crate cedar;

use cedar::dom::{Object, text};

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

// view content =
//   div []
//     [ input [ placeholder "Text to reverse", onInput NewContent, myStyle ] []
//     , div [ myStyle ] [ text (String.reverse content) ]
//     ]

// myStyle =
//   style
//     [ ("width", "100%")
//     , ("height", "40px")
//     , ("padding", "10px 0")
//     , ("font-size", "2em")
//     , ("text-align", "center")
//     ]

fn view(model: &Model) -> Object<Message> {
    div!(
        [],
        [
            input!([], [])
                .placeholder("Text to reverse!".into())
                .change(Message::NewContent),
            div!([], [text(model.chars().rev().collect::<String>())]),
        ]
    )
}

fn main() {
    cedar::program("--".into(), update, view)
}
