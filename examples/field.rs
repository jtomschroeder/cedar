
extern crate cedar;

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

use cedar::dom;

// view content =
//   div []
//     [ input [ placeholder "Text to reverse", onInput NewContent, myStyle ] []
//     , div [ myStyle ] [ text (String.reverse content) ]
//     ]

fn view(model: &Model) -> dom::Object<Message> {
    dom::div(vec![
        dom::field()
            .placeholder("Text to reverse!".into())
            .change(Message::NewContent),
        dom::label(model.chars().rev().collect()),
    ])
}

fn main() {
    cedar::program("--".into(), update, view)
}
