
extern crate cedar;

#[macro_use]
extern crate dom;

type Model = i32;

enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: Message) -> Model {
    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> cedar::dom::Node {
    use dom::Kind;
    use dom::Attribute::*;
    use dom::Operation;

    node![Kind::Stack => node![Kind::Button]
                       , node![Kind::Label |> Text(model.to_string())]
                       , node![Kind::Button]
         ]
}

fn main() {
    cedar::Program::new(0, update, view).run()
}