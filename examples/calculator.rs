use cedar::prelude::*;

type Model = String; // TODO: f64?

#[derive(PartialEq, Debug)]
enum Message {
    Click(String),
}

fn update(model: Model, message: &Message) -> Model {
    println!("message: {:?}", message);

    model
}

type Object = cedar::dom::Object<Message>;

// TODO: input => onChange
// TODO: click => onClick

struct Row {
    children: Vec<Object>,
}

impl Component<Message> for Row {
    fn render(self) -> Object {
        sml! {
            (div (@ (class "flex flex-row flex-wrap flex-grow"))
                (div (@ (class "flex flex-grow w-100")) { self.children })
            )
        }
    }
}

struct Button<'s> {
    value: &'s str,
}

impl<'s> Component<Message> for Button<'s> {
    fn render(self) -> Object {
        sml! {
            (div (@ (class "inline-flex flex-grow w-25"))
                (button
                    (@ (class "flex-grow") (click Message::Click(self.value.into())))
                    { self.value })
            )
        }
    }
}

fn view(model: &Model) -> Object {
    sml! {
        (div (@ (class "flex flex-column flex-wrap vh-100"))
            (div (@ (class "flex-none bg-gray white tr w-100"))
                (div (@ (class "f2 pa3")) { model.as_str() })
            )

            (& Row
                (& Button (@ (value "AC")))
                (& Button (@ (value "+/-")))
                (& Button (@ (value "%")))
                (& Button (@ (value "÷")))
            )

            (& Row
                (& Button (@ (value "7")))
                (& Button (@ (value "8")))
                (& Button (@ (value "9")))
                (& Button (@ (value "X")))
            )

            (& Row
                (& Button (@ (value "4")))
                (& Button (@ (value "5")))
                (& Button (@ (value "6")))
                (& Button (@ (value "-")))
            )

            (& Row
                (& Button (@ (value "1")))
                (& Button (@ (value "2")))
                (& Button (@ (value "3")))
                (& Button (@ (value "+")))
            )

            (& Row
                (& Button (@ (value "0")))
                (& Button (@ (value ".")))
                (& Button (@ (value "=")))
            )
        )
    }
}

fn main() {
    cedar::app(Model::from("0"), update, view)
}
