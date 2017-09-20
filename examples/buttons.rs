
extern crate cedar;

use cedar::dom;
use cedar::dom::Builder;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
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

// TODO: create macro language a la JSX in React for defining DOM

// <stack>
// <button></button>
// <label></button>
// <button></button>
// </stack>

// or a lisp?
// (stack [(button), (label), (button)])

// or a la elm?
// view model =
//   div []
//     [ button [ onClick Decrement ] [ text "-" ]
//     , div [] [ text (toString model) ]
//     , button [ onClick Increment ] [ text "+" ]
//     ]

fn view(model: &Model) -> dom::Object<Message> {
    dom::stack()
        .add(dom::button().text("+".into()).click(Message::Increment))
        .add(dom::label().text(model.to_string()))
        .add(dom::button().text("-".into()).click(Message::Decrement))
}

fn main() {
    cedar::program(0, update, view)
}