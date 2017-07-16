
extern crate cedar;

#[macro_use]
extern crate dom;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: Message) -> Model {
    // println!("UPDATE! {} :: {:?}", model, message);

    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> cedar::backend::Node<Message> {
    use cedar::backend::Kind::*;
    use cedar::backend::Attribute::*;

    node![(Stack, vec![]) 
            => node![(Button, vec![Click(Message::Increment)])]
             , node![(Label, vec![Text(model.to_string())])]
             , node![(Button, vec![Click(Message::Decrement)])]
         ]
}

fn main() {
    cedar::Program::new(0, update, view).run()
}