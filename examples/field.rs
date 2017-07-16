
extern crate cedar;

type Model = String;

#[derive(PartialEq, Clone, Debug)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: Message) -> Model {
    match message {
        Message::NewContent(content) => content,
    }
}

fn view(model: &Model) -> cedar::dom::Node<Message> {
    use cedar::dom;
    dom::stack()
        .add(dom::field()
                 .placeholder("Text to reverse!".into())
                 .change(Message::NewContent))
        .add(dom::label().text(model.chars().rev().collect()))
        .create()
}

fn main() {
    cedar::program("--".into(), update, view)
}
