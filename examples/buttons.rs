
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

fn view(model: &Model) -> cedar::backend::Node {
    use cedar::backend::Kind::*;
    use cedar::backend::Attribute::*;

    node![(Stack, vec![]) 
            => node![(Button, vec![])]
                , node![(Label, vec![Text("!".into())])]
                , node![(Button, vec![])]
            ]
}

fn main() {
    cedar::Program::new(0, update, view).run()
}