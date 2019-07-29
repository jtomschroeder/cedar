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
        (div (@ (class "flex h-15 items-center")) { children })
    }
}

fn button(_attrs: Vec<Attribute<Message>>, children: Vec<Object>) -> Object {
    sml! {
        (div (@ (class "w-25")) { children })
    }
}

fn view(_: &Model) -> Object {
    sml! {
        (div (@ (class "tc vh-100"))
            (div (@ (class "w-100 tr")) { "0" })

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
