
extern crate cedar;

use cedar::{Application, View};

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

fn view() -> View<Model, Message> {
    View::new()
        .button(|button| {
            button.text("+")
                .click(|| Message::Increment)
        })
        .label(|label| label.text(|model: Model| model.to_string()))
        .button(|button| {
            button.text("-")
                .click(|| Message::Decrement)
        })
}

fn main() {
    let app = Application::new(0, update, view);
    app.run()
}
