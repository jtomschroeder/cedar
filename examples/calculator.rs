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

impl CustomComponent<Message> for Row {
    fn render(self) -> Object {
        sml! {
            (div (@ (class "component-button-panel"))
                (div { self.children })
            )
        }
    }
}

struct Button {
    children: Vec<Object>,
}

impl CustomComponent<Message> for Button {
    fn render(self) -> Object {
        sml! {
            (div (@ (class "component-button"))
                (button { self.children })
            )
        }
    }
}

fn view(_: &Model) -> Object {
    sml! {
        (div (@ (class "flex flex-wrap flex-column vh-100"))
            (div (@ (class "bg-gray white tr w-100"))
                (div (@ (class "pa2 f2")) { "0" })
            )

            (& Row
                (& Button { "C" })
                (& Button { "+/-" })
                (& Button { "%" })
                (& Button { "÷" })

            (& Row
                (& Button { "7" })
                (& Button { "8" })
                (& Button { "9" })
                (& Button { "X" })
            )
            )

            (& Row
                (& Button { "4" })
                (& Button { "5" })
                (& Button { "6" })
                (& Button { "-" })
            )

            (& Row
                (& Button { "1" })
                (& Button { "2" })
                (& Button { "3" })
                (& Button { "+" })
            )

            (& Row
                (& Button { "0" })
                (& Button { "." })
                (& Button { "=" })
            )
        )
    }
}

fn main() {
    cedar::app((), update, view)
}
