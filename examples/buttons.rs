use cedar::sml;

type Model = i32;

#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: &Message) -> Model {
    match message {
        &Message::Increment => model + 1,
        &Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> cedar::dom::Object<Message> {
    sml! {
        (div
            (button (@ (click Message::Increment)) {"+"})
            (div { model.to_string() })
            (button (@ (click Message::Decrement)) {"-"})
        )
    }
}

fn main() {
    cedar::app(0, update, view)
}
