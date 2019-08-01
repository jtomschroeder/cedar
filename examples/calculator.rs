use cedar::prelude::*;

type Model = String; // TODO: f64?

#[derive(PartialEq, Debug)]
enum Message {
    Click(String),
}

fn update(model: Model, message: &Message) -> Model {
    println!("message: {:?}", message);

    // TODO!

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

#[derive(Default)]
struct Button<'s> {
    value: &'s str,
    wide: bool,
    color: &'s str,
}

impl<'s> Component<Message> for Button<'s> {
    fn render(self) -> Object {
        let container_class = format!(
            "inline-flex flex-grow {}",
            if self.wide { "w-50" } else { "w-25" },
        );

        let button_class = format!("flex-grow bg-{}", self.color);

        sml! {
            (div (@ (class container_class))
                (button
                    (@ (class button_class) (click Message::Click(self.value.into())))
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
                (& Button (@ (value "AC") (wide false) (color "lightgray")))
                (& Button (@ (value "+/-") (wide false) (color "lightgray")))
                (& Button (@ (value "%") (wide false) (color "lightgray")))
                (& Button (@ (value "÷") (wide false) (color "orange")))
            )

            (& Row
                (& Button (@ (value "7") (wide false) (color "lightgray")))
                (& Button (@ (value "8") (wide false) (color "lightgray")))
                (& Button (@ (value "9") (wide false) (color "lightgray")))
                (& Button (@ (value "X") (wide false) (color "orange")))
            )

            (& Row
                (& Button (@ (value "4") (wide false) (color "lightgray")))
                (& Button (@ (value "5") (wide false) (color "lightgray")))
                (& Button (@ (value "6") (wide false) (color "lightgray")))
                (& Button (@ (value "-") (wide false) (color "orange")))
            )

            (& Row
                (& Button (@ (value "1") (wide false) (color "lightgray")))
                (& Button (@ (value "2") (wide false) (color "lightgray")))
                (& Button (@ (value "3") (wide false) (color "lightgray")))
                (& Button (@ (value "+") (wide false) (color "orange")))
            )

            (& Row
                (& Button (@ (value "0") (wide true) (color "lightgray")))
                (& Button (@ (value ".") (wide false) (color "lightgray")))
                (& Button (@ (value "=") (wide false) (color "orange")))
            )
        )
    }
}

fn main() {
    cedar::app(Model::from("0"), update, view)
}
