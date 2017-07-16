
use std::marker::PhantomData;
use std::sync::Arc;

use dom;
use super::{View, Window, Label, Stack, Button};
use cacao::widget::Widget;
use stream::Stream;

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

#[derive(PartialEq, Debug)]
pub enum Kind {
    Stack,
    Button,
    Label,
}

#[derive(PartialEq, Debug)]
pub enum Attribute<S> {
    Text(String),
    Click(S),
}

pub type Attributes<S> = Vec<Attribute<S>>;

pub type Value<S> = (Kind, Attributes<S>);
pub type Node<S> = dom::Node<Value<S>>;

fn create<S: Clone + 'static>(stream: Stream<S>, node: Node<S>) -> Box<Widget<S>> {
    let mut widget: Box<Widget<S>> = match node.value.0 {
        Kind::Label => Box::new(Label::new()),
        Kind::Button => Box::new(Button::new(stream.clone())), 
        Kind::Stack => Box::new(Stack::new()),
    };

    let attrs = node.value.1;
    widget.update(attrs);

    for child in node.children.into_iter() {
        widget.add(create(stream.clone(), child));
    }

    widget
}

// TODO: use `removeFromSuperview()` to 'delete' nodes
// TODO: maintain `tree` of widgets here instead of in each widget

impl<S, M, U, V> Program<S, M, U, V>
    where S: Clone + Send + 'static,
          M: Send + 'static,
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

        stack.add(create(stream.clone(), node));

        // let mut view = self.view.view();

        // let mut model = self.model;
        // view.update(&model);

        // Use `Option` to allow for move/mutation in FnMut `run`
        let mut model = Some(model);

        // let mut stack = Arc::new(stack);

        let mut update = self.update;
        app.run(move || loop {
                    let message = stream.pop();

                    // println!("msg: {:?}", message);

                    let m = update.update(model.take().unwrap(), message);

                    let node = view(&m);

                    // stack.add(create(stream.clone(), node));

                    model = Some(m);
                })
    }
}
