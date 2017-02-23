
extern crate cedar;

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

fn view() -> cedar::View<Model, Message> {
    cedar::View::new()
        .button(|button| {
            button.text("+")
                .click(|| Message::Increment)
        })
        .label(|label| label.text(Model::to_string))
        .button(|button| {
            button.text("-")
                .click(|| Message::Decrement)
        })
}

fn main() {
    cedar::Application::new(0, update, view).run()
}
