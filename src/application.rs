
use std;
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;

use super::cacao;
use super::View;

pub trait Viewable<M, S> {
    fn view(&mut self) -> View<M, S>;
}

impl<M, S, F> Viewable<M, S> for F
    where F: FnMut() -> View<M, S>
{
    fn view(&mut self) -> View<M, S> {
        self()
    }
}

pub trait Update<M, S> {
    fn update(&mut self, model: M, message: S) -> M;
}

impl<M, S, F> Update<M, S> for F
    where F: FnMut(M, S) -> M
{
    fn update(&mut self, model: M, message: S) -> M {
        self(model, message)
    }
}

pub struct Application<S, M, U, V> {
    model: M,
    update: U,
    view: V,
    message: PhantomData<S>,
}

impl<S, M, U, V> Application<S, M, U, V> {
    pub fn new(model: M, update: U, view: V) -> Self {
        Application {
            model: model,
            update: update,
            view: view,
            message: PhantomData,
        }
    }
}

impl<S, M, U, V> Application<S, M, U, V>
    where S: 'static,
          M: Clone + 'static,
          U: Update<M, S> + Send + 'static,
          V: Viewable<M, S>
{
    pub fn run(mut self) {
        let app = cacao::Application::new();

        let mut view = self.view.view();

        let mut model = self.model;
        view.update(model.clone());

        let view = Arc::new(Mutex::new(view));

        use crossbeam;
        use std::sync::{Arc, Barrier};

        let barrier = Arc::new(Barrier::new(2));

        let mut update = self.update;
        // crossbeam::scope(|scope| {
        //
        //     scope.spawn(|| {
        //         // barrier.wait();
        //         app.run()
        //     });
        //
        //     // let c = barrier.clone();
        //     scope.spawn(move || {
        //         // c.wait();
        //         loop {
        //             if let Ok(mut view) = view.lock() {
        //                 let message = view.stream().pop();
        //                 model = update.update(model.clone(), message);
        //                 view.update(model.clone());
        //             }
        //         }
        //     });
        // });

        // std::thread::spawn();

        app.run(move || loop {
            if let Ok(mut view) = view.lock() {
                let message = view.stream().pop();
                model = update.update(model.clone(), message);
                view.update(model.clone());
            }
        })
    }
}
