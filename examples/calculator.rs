use cedar::dom::Attribute;
use cedar::prelude::*;

type Model = ();

#[derive(PartialEq)]
enum Message {}

fn update(model: Model, _message: &Message) -> Model {
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
                (button (@ (class "flex-grow")) { self.value })
            )
        }
    }
}

fn view(_: &Model) -> Object {
    sml! {
        (div (@ (class "flex flex-column flex-wrap vh-100"))
            (div (@ (class "flex-none bg-gray white tr w-100"))
                (div (@ (class "f2 pa3")) { "0" })
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
    cedar::app((), update, view)
}
