
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

type Widget = dom::Object<Message>;

fn words(line: &str) -> Vec<Widget> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|w| dom::div().add(dom::text(w)))
        .collect()
}

fn view(model: &Model) -> Widget {
    dom::div()
        .add(dom::input().placeholder("Words!").input(
            Message::NewContent,
        ))
        .add(dom::div().children(words(model)))
}

fn main() {
    cedar::program("".into(), update, view)
}
