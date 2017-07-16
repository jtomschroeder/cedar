
use tree;
use super::{View, Window, Label, Stack, Button, TextField};
use cacao::widget::Widget;
use stream::Stream;
use atomic_box::AtomicBox;

#[derive(PartialEq, Clone, Debug)]
pub enum Kind {
    Stack,
    Button,
    Label,
    Field,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Attribute<S> {
    Text(String),
    Click(S),
    Placeholder(String),
    Change(fn(String) -> S),
}

pub type Attributes<S> = Vec<Attribute<S>>;

pub type Value<S> = (Kind, Attributes<S>);
pub type Node<S> = tree::Node<Value<S>>;

struct Vertex<S> {
    widget: AtomicBox<Box<Widget<S>>>,
    children: Vec<Vertex<S>>,
}

type Tree<S> = Vec<Vertex<S>>;

fn create<S: Clone + 'static>(stream: Stream<S>, node: Node<S>) -> Vertex<S> {
    let mut widget: Box<Widget<S>> = match node.value.0 {
        Kind::Label => Box::new(Label::new()),
        Kind::Button => Box::new(Button::new(stream.clone())), 
        Kind::Stack => Box::new(Stack::new()),
        Kind::Field => Box::new(TextField::new(stream.clone())),
    };

    let attrs = node.value.1;
    widget.update(attrs);

    let mut children = vec![];
    for child in node.children.into_iter() {
        let child = create(stream.clone(), child);
        widget.add(&child.widget);
        children.push(child);
    }

    Vertex {
        widget: AtomicBox::new(widget),
        children,
    }
}

// TODO: use `removeFromSuperview()` to 'delete' nodes

use std::fmt::Debug;

fn comparator<S>(t: &Node<S>, u: &Node<S>) -> Option<tree::Difference>
    where S: PartialEq
{
    if t.value.0 != u.value.0 {
        Some(tree::Difference::Kind)
    } else if t.value.1 != u.value.1 {
        Some(tree::Difference::Value)
    } else {
        None
    }
}

type Change<S> = tree::Change<Value<S>>;
type Changeset<S> = tree::Changeset<Value<S>>;

fn traverse<S: Debug>(tree: &mut Tree<S>, change: Change<S>) {
    if change.0.is_empty() {
        return;
    }

    let (mut path, op) = change;
    let location = path.remove(0);

    if path.is_empty() {
        let widget = &mut tree[location.index].widget;

        use tree::Operation::*;
        match op {
            Update((_, attrs)) => widget.update(attrs),
            op => panic!("Not yet implemented! {:?}", op),
        }
    } else {
        traverse(&mut tree[location.index].children, (path, op));
    }
}

pub fn program<S, M, U, V>(model: M, mut update: U, view: V)
    where S: Clone + Send + 'static + PartialEq + Debug,
          M: Send + 'static + Debug,
          U: ::Update<M, S> + Send + 'static,
          V: Send + Fn(&M) -> Node<S> + 'static
{
    let app = super::Application::new(); // TODO: enforce `app` created first

    let stream = Stream::new();

    let (window, mut stack) = Window::new("cedar");

    let node = view(&model);

    let vertex = create(stream.clone(), node.clone());
    stack.add(&vertex.widget);

    let mut tree = vec![vertex];

    // Use `Option` to allow for move/mutation in FnMut `run`
    let mut model = Some(model);
    let mut node = Some(node);

    app.run(move || loop {
                let message = stream.pop();

                // println!("msg: {:?}", message);

                let m = update.update(model.take().unwrap(), message);

                let new = view(&m);

                // println!("node: {:?}", new);

                let old = node.take().unwrap();
                let changeset = tree::diff(vec![old], vec![new.clone()], comparator);

                // println!("diff: {:?}", changeset);

                for change in changeset.into_iter() {
                    traverse(&mut tree, change);
                }

                node = Some(new);
                model = Some(m);
            })

}
