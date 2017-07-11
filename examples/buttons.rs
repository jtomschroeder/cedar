
extern crate cedar;

type Model = i32;

type Update = fn(Model, Message) -> Model;
type View = fn(&Model) -> Node;

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

// node : String -> List Attribute -> List Html -> Html

#[derive(Clone, Debug)]
enum Object {
    Div,
    Button,
    Text(String),
}

#[derive(Clone, Debug)]
struct Node {
    // tag, a list of HTML attributes, and a list of children
    object: Object,
    children: Vec<Box<Node>>,
}

fn div(children: &[Box<Node>]) -> Node {
    Node {
        object: Object::Div,
        children: children.into(),
    }
}

fn button() -> Node {
    Node {
        object: Object::Button,
        children: vec![],
    }
}

fn text(txt: String) -> Node {
    Node {
        object: Object::Text(txt),
        children: vec![],
    }
}

// fn view() -> cedar::View<Model, Message> {
//     cedar::View::new()
//         .button(|button| button.text("+").click(|| Message::Increment))
//         .label(|label| label.text(Model::to_string))
//         .button(|button| button.text("-").click(|| Message::Decrement))
// }

// view model =
//   div []
//     [ button [ onClick Decrement ] [ text "-" ]
//     , div [] [ text (toString model) ]
//     , button [ onClick Increment ] [ text "+" ]
//     ]

fn view(model: &Model) -> Node {
    div(&[Box::new(button()),
          Box::new(text(model.to_string())),
          Box::new(button())])
}

#[derive(Clone, Debug)]
enum Change {
    Replace,
}

fn diff(old: Vec<Node>, new: Vec<Node>) -> Vec<Change> {

    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: REMOVE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    update properties and keep going

    // Traverse by 'level'

    // if old.object == new.object {}

    let mut changes = vec![];

    for (old, new) in old.iter().zip(&new) {}

    changes
}

fn program(model: Model, update: Update, view: View) {
    let v1 = view(&model);
    // println!("{:?} :: {:#?}", model, v1);

    let model = update(model, Message::Increment);
    let v2 = view(&model);
    // println!("{:?} :: {:#?}", model, v2);

    let model = update(model, Message::Decrement);
    let v3 = view(&model);
    // println!("{:?} :: {:#?}", model, v3);

    println!("{:#?}", diff(vec![v1], vec![v2.clone()]));
}

fn main() {
    // cedar::Program::new(0, update, view).run()
    program(0, update, view)
}
