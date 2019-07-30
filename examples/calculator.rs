use cedar::prelude::*;
use cedar::dom::Attribute;

type Model = ();

#[derive(PartialEq)]
enum Message {}

fn update(model: Model, _message: &Message) -> Model {
    model
}

type Object = cedar::dom::Object<Message>;

// TODO: input => onChange
// TODO: click => onClick

fn row(_attrs: Vec<Attribute<Message>>, children: Vec<Object>) -> Object {
    sml! {
        (div (@ (class "component-button-panel"))
            (div { children })
        )
    }
}

fn button(_attrs: Vec<Attribute<Message>>, children: Vec<Object>) -> Object {
    sml! {
        (div (@ (class "component-button"))
            (button { children })
        )
    }
}

fn view(_: &Model) -> Object {
    sml! {
        (div (@ (class "flex flex-wrap flex-column vh-100"))
            (div (@ (class "bg-gray white tr w-100"))
                (div (@ (class "pa2 f2")) { "0" })
            )

            (& row
                (& button { "C" })
                (& button { "+/-" })
                (& button { "%" })
                (& button { "÷" })
            )

            (& row
                (& button { "7" })
                (& button { "8" })
                (& button { "9" })
                (& button { "X" })
            )

            (& row
                (& button { "4" })
                (& button { "5" })
                (& button { "6" })
                (& button { "-" })
            )

            (& row
                (& button { "1" })
                (& button { "2" })
                (& button { "3" })
                (& button { "+" })
            )

            (& row
                (& button { "0" })
                (& button)
                (& button { "." })
                (& button { "=" })
            )
        )
    }
}

fn main() {
    cedar::app((), update, view)
}
