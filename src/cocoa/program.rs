
use std::marker::PhantomData;

use dom;
use super::{View, Window, Label, Stack, Button};
use cacao::widget::Widget;

pub trait Viewable<M, S> {
    fn view(&self, &M) -> Node<S>;
}

impl<M, S, F> Viewable<M, S> for F
    where F: Fn(&M) -> Node<S>
{
    fn view(&self, model: &M) -> Node<S> {
        self(model)
    }
}

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

fn create<S: 'static>(node: Node<S>) -> Box<Widget<S>> {
    let mut widget: Box<Widget<S>> = match node.value.0 {
        Kind::Label => Box::new(Label::new()),
        Kind::Button => Box::new(Button::new()), 
        Kind::Stack => Box::new(Stack::new()),
    };

    let attrs = node.value.1;
    widget.update(attrs);

    for child in node.children.into_iter() {
        widget.add(create(child));
    }

    widget
}

// TODO: use `removeFromSuperview()` to 'delete' nodes
// TODO: maintain `tree` of widgets here instead of in each widget

impl<S, M, U, V> Program<S, M, U, V>
    where S: Send + 'static,
          M: Send + 'static,
          U: ::Update<M, S> + Send + 'static,
          V: Viewable<M, S>
{
    pub fn run(self) {
        let app = super::Application::new(); // TODO: enforce `app` created first

        let model = self.model;

        let (window, mut stack) = Window::new("cedar");

        let view = self.view;
        let node = view.view(&model);

        stack.add(create(node));

        // let mut view = self.view.view();

        // let mut model = self.model;
        // view.update(&model);

        // let mut update = self.update;
        app.run(move || loop {
                    // let message = view.stream().pop();
                    // model = update.update(&model, message);
                    // view.update(&model);
                })
    }
}
