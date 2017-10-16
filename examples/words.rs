
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
    // TODO: split line on spaces and make label for each word
    vec![]
}

fn view(model: &Model) -> Widget {
    dom::stack(vec![
        dom::field().placeholder("Words!".into()).change(
            Message::NewContent
        ),

        dom::stack(words(model)),
    ])
}

fn main() {
    cedar::program("--".into(), update, view)
}
