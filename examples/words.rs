
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
        .map(|w| dom::div(vec![dom::label(w.into())]))
        .collect()
}

fn view(model: &Model) -> Widget {
    dom::div(vec![
        dom::input().placeholder("Words!".into()).change(
            Message::NewContent
        ),

        dom::div(words(model)),
    ])
}

fn main() {
    cedar::program("".into(), update, view)
}
