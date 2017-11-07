
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

fn view(model: &Model) -> Object<Message> {
    let style = vec![
        ("width", "100%"),
        ("height", "40px"),
        ("padding", "10px 0"),
        ("font-size", "2em"),
        ("text-align", "center"),
    ];

    div().children(vec![
        input()
            .placeholder("Text to reverse!")
            .style(style.clone())
            .input(Message::NewContent),
        div().style(style).add(text(
            model
                .chars()
                .rev()
                .collect::<String>(),
        )),
    ])
}

fn main() {
    cedar::program("".into(), update, view)
}
