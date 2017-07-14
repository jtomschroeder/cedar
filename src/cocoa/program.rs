
use std::marker::PhantomData;

use dom;
use super::{View, Window, Label, Stack, Button};
use cacao::widget::Widget;

pub trait Viewable<S> {
    fn view(&self) -> dom::Node;
}

impl<S, F> Viewable<S> for F
    where F: Fn() -> dom::Node
{
    fn view(&self) -> dom::Node {
        self()
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

fn create(node: dom::Node) -> Box<Widget> {
    let mut widget: Box<Widget> = match node.kind {
        dom::Kind::Label => Box::new(Label::new()),
        dom::Kind::Button => Box::new(Button::new()), 
        dom::Kind::Stack => Box::new(Stack::new()),
    };

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
          V: Viewable<S>
{
    pub fn run(self) {
        let app = super::Application::new(); // TODO: enforce `app` created first

        let mut window = Window::new("cedar");

        let view = self.view;
        let node = view.view();

        window.add(create(node));

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
