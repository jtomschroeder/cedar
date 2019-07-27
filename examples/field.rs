use cedar::sml;

type Model = String;

#[derive(PartialEq)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: &Message) -> Model {
    match message {
        &Message::NewContent(ref content) => content.clone(),
    }
}

fn view(model: &Model) -> cedar::dom::Object<Message> {
    let field: String = model.chars().rev().collect();

    sml! {
        (div (@ (class "tc f2"))
            (input (@ (class "w-100 tc pa3") (input Message::NewContent)))
            (div {field})
        )
    }
}

fn main() {
    cedar::app("".into(), update, view)
}
