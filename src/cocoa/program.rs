
use std::marker::PhantomData;
use std::sync::Arc;

use dom;
use super::{View, Window, Label, Stack, Button};
use cacao::widget::Widget;
use stream::Stream;
use atomic_box::AtomicBox;

pub struct Program<S, M, U, V> {
    model: M,
    update: U,
    view: V,
    message: PhantomData<S>,
}

impl<S, M, U, V> Program<S, M, U, V> {
    pub fn new(model: M, update: U, view: V) -> Self {
        Program {
            model: model,
            update: update,
            view: view,
            message: PhantomData,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Kind {
    Stack,
    Button,
    Label,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Attribute<S> {
    Text(String),
    Click(S),
}

pub type Attributes<S> = Vec<Attribute<S>>;

pub type Value<S> = (Kind, Attributes<S>);
pub type Node<S> = dom::Node<Value<S>>;

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

fn comparator<S>(t: &Node<S>, u: &Node<S>) -> Option<dom::Difference>
    where S: PartialEq
{
    if t.value.0 != u.value.0 {
        Some(dom::Difference::Kind)
    } else if t.value.1 != u.value.1 {
        Some(dom::Difference::Value)
    } else {
        None
    }
}

type Change<S> = dom::Change<Value<S>>;
type Changeset<S> = dom::Changeset<Value<S>>;

fn traverse<S: Debug>(tree: &mut Tree<S>, change: Change<S>) {
    if change.0.is_empty() {
        return;
    }

    let (mut path, op) = change;
    let location = path.remove(0);

    if path.is_empty() {
        let widget = &mut tree[location.index].widget;

        use dom::Operation::*;
        match op {
            Update((_, attrs)) => widget.update(attrs),
            op => panic!("Not yet implemented! {:?}", op),
        }
    } else {
        traverse(&mut tree[location.index].children, (path, op));
    }
}

impl<S, M, U, V> Program<S, M, U, V>
    where S: Clone + Send + 'static + PartialEq + Debug,
          M: Send + 'static + Debug,
          U: ::Update<M, S> + Send + 'static,
          V: Send + Fn(&M) -> Node<S> + 'static
{
    pub fn run(self) {
        let app = super::Application::new(); // TODO: enforce `app` created first

        let stream = Stream::new();

        let model = self.model;

        let (window, mut stack) = Window::new("cedar");

        let view = self.view;
        let node = view(&model);

        let vertex = create(stream.clone(), node.clone());
        stack.add(&vertex.widget);

        let mut tree = vec![vertex];

        // Use `Option` to allow for move/mutation in FnMut `run`
        let mut model = Some(model);
        let mut node = Some(node);

        // let mut stack = Arc::new(stack);

        let mut update = self.update;
        app.run(move || loop {
                    let message = stream.pop();

                    // println!("msg: {:?}", message);

                    let m = update.update(model.take().unwrap(), message);

                    let new = view(&m);

                    // println!("node: {:?}", new);

                    let old = node.take().unwrap();
                    let changeset = dom::diff(vec![old], vec![new.clone()], comparator);

                    // println!("diff: {:?}", changeset);

                    for change in changeset.into_iter() {
                        traverse(&mut tree, change);
                    }

                    node = Some(new);
                    model = Some(m);
                })
    }
}
