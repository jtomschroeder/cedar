
// use std::marker::PhantomData;

// use super::View;

// pub trait Viewable<M, S> {
//     fn view(&mut self) -> View<M, S>;
// }

// impl<M, S, F> Viewable<M, S> for F
//     where F: FnMut() -> View<M, S>
// {
//     fn view(&mut self) -> View<M, S> {
//         self()
//     }
// }

// pub struct Program<S, M, U, V> {
//     model: M,
//     update: U,
//     view: V,
//     message: PhantomData<S>,
// }

// impl<S, M, U, V> Program<S, M, U, V> {
//     pub fn new(model: M, update: U, view: V) -> Self {
//         Program {
//             model: model,
//             update: update,
//             view: view,
//             message: PhantomData,
//         }
//     }
// }

// impl<S, M, U, V> Program<S, M, U, V>
//     where S: Send + 'static,
//           M: Send + 'static,
//           U: ::Update<M, S> + Send + 'static,
//           V: Viewable<M, S>
// {
//     pub fn run(mut self) {
//         let app = super::Application::new(); // TODO: enforce `app` created first

//         let mut view = self.view.view();

//         let mut model = self.model;
//         view.update(&model);

//         let mut update = self.update;
//         app.run(move || if let Some(msg) = view.stream().try_pop() {
//             model = update.update(&model, msg);
//             view.update(&model);
//         })
//     }
// }

use gtk;
use gtk::prelude::*;

use dom;
use std::fmt::Debug;

use super::{Window, Button, Label, Stack};
use stream::Stream;

use super::widget::{Widget, NWidget};
use atomic_box::AtomicBox;

// type Wdgt = Box<gtk::IsA<gtk::Widget>>;

struct Vertex<S> {
    widget: AtomicBox<NWidget<S>>,
    children: Vec<Vertex<S>>,
}

type Tree<S> = Vec<Vertex<S>>;

fn create<S: Clone + 'static>(stream: Stream<S>, node: dom::Object<S>) -> Vertex<S> {
    let (kind, attributes) = (node.value.0, node.value.1);
    let mut widget: NWidget<S> = match kind {
        dom::Kind::Label => NWidget::Label(Label::new()),
        dom::Kind::Button => NWidget::Button(Button::new(stream.clone())), 
        dom::Kind::Stack => NWidget::Stack(Stack::new()),
        // dom::Kind::Field => Box::new(TextField::new(stream.clone())),
        k => panic!("Not yet implemented! {:?}", k),
    };

    widget.update(attributes);

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

fn patch<S: Debug>(tree: &mut Tree<S>, (mut path, op): dom::Change<S>) {
    if path.is_empty() {
        return;
    }

    let location = path.remove(0);
    if path.is_empty() {
        let widget = &mut tree[location.index].widget;

        use tree::Operation::*;
        match op {
            // Update((_, attrs)) => widget.update(attrs),
            op => panic!("Not yet implemented! {:?}", op),
        }
    } else {
        patch(&mut tree[location.index].children, (path, op));
    }
}

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
    where S: Clone + Send + 'static + PartialEq + Debug,
          M: Send + 'static + Debug
{
    let app = super::Application::new(); // TODO: enforce `app` created first

    let stream: Stream<S> = Stream::new();
    let (window, mut stack) = Window::new("cedar");

    let button: Button<S> = Button::new(stream.clone());
    let button: Box<Widget<S>> = Box::new(button);

    // button.add(&stack);
    // stack.add(&button);

    let node = view(&model);

    let vertex = create(stream.clone(), node.clone());
    stack.add(&vertex.widget);

    let mut tree = vec![vertex];

    // Use `Option` to allow for move/mutation in FnMut `run`
    let mut model = Some(model);
    // let mut node = Some(node);

    app.run(move || if let Some(message) = stream.try_pop() {
                println!("msg: {:?}", message);
                let m = update(model.take().unwrap(), message);

                model = Some(m);
            });
}