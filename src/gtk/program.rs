
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

use dom;
use std::fmt::Debug;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
    where S: Clone + Send + 'static + PartialEq + Debug,
          M: Send + 'static + Debug
{
    let app = super::Application::new(); // TODO: enforce `app` created first

    // let stream = Stream::new();

    // let (_window, mut stack) = Window::new("cedar");

    // let node = view(&model);

    // let vertex = create(stream.clone(), node.clone());
    // stack.add(&vertex.widget);

    // let mut tree = vec![vertex];

    // // Use `Option` to allow for move/mutation in FnMut `run`
    // let mut model = Some(model);
    // let mut node = Some(node);

    app.run(move || {})

}