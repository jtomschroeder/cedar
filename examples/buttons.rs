
extern crate cedar;

#[macro_use]
extern crate tree;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
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

fn view(model: &Model) -> cedar::dom::Node<Message> {
    use cedar::dom::Kind::*;
    use cedar::dom::Attribute::*;

    node![(Stack, vec![]) 
            => node![(Button, vec![Text("+".into()), 
                                   Click(Message::Increment)])]
             , node![(Label,  vec![Text(model.to_string())])]
             , node![(Button, vec![Text("-".into()), 
                                   Click(Message::Decrement)])]
         ]
}

fn main() {
    cedar::program(0, update, view)
}