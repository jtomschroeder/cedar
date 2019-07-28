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

fn view(model: &Model) -> Object {
    let row = |attrs, children| sml! { (div (@ (class "flex h-15 items-center")) { children }) };

    sml! {
        (div (@ (class "tc vh-100"))
            (div (@ (class "w-100 tr")) { "0" })

            (& row
                (div (@ (class "w-25")) { "C" })
                (div (@ (class "w-25")) { "+/-" })
                (div (@ (class "w-25")) { "%" })
                (div (@ (class "w-25")) { "÷" })
            )

            (div (@ (class "flex h-15 items-center"))
                (div (@ (class "w-25")) { "7" })
                (div (@ (class "w-25")) { "8" })
                (div (@ (class "w-25")) { "9" })
                (div (@ (class "w-25")) { "X" })
            )

            (div (@ (class "flex h-15 items-center"))
                (div (@ (class "w-25")) { "4" })
                (div (@ (class "w-25")) { "5" })
                (div (@ (class "w-25")) { "6" })
                (div (@ (class "w-25")) { "-" })
            )

            (div (@ (class "flex h-15 items-center"))
                (div (@ (class "w-25")) { "1" })
                (div (@ (class "w-25")) { "2" })
                (div (@ (class "w-25")) { "3" })
                (div (@ (class "w-25")) { "+" })
            )

            (div (@ (class "flex h-15 items-center"))
                (div (@ (class "w-25")) { "0" })
                (div (@ (class "w-25")))
                (div (@ (class "w-25")) { "." })
                (div (@ (class "w-25")) { "=" })
            )
        )
    }
}

fn main() {
    cedar::app((), update, view)
}
