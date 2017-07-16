
extern crate cedar;

#[macro_use]
extern crate tree;

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
    // cedar::View::new()
    //     .field(|field| {
    //                field
    //                    .placeholder("Text to reverse")
    //                    .change(|s| Message::NewContent(s.into()))
    //            })
    //     .label(|label| label.text(|m: &Model| m.chars().rev().collect()));

    use cedar::dom::Kind::*;
    use cedar::dom::Attribute::*;

    node![(Stack, vec![]) 
            => node![(Field, vec![Placeholder("".into()),
                                  Change(Message::NewContent)])]
             , node![(Label, vec![Text(model.chars().rev().collect())])]
         ]
}

fn main() {
    cedar::program("--".into(), update, view)
}
