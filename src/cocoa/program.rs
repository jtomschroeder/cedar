
use std::marker::PhantomData;

use super::View;

pub trait Viewable<M, S> {
    fn view(&self) -> View<M, S>;
}

impl<M, S, F> Viewable<M, S> for F
    where F: Fn() -> View<M, S>
{
    fn view(&self) -> View<M, S> {
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

impl<S, M, U, V> Program<S, M, U, V>
    where S: Send + 'static,
          M: Send + 'static,
          U: ::Update<M, S> + Send + 'static,
          V: Viewable<M, S>
{
    pub fn run(self) {
        let app = super::Application::new(); // TODO: enforce `app` created first

        let mut view = self.view.view();

        let mut model = self.model;
        view.update(&model);

        let mut update = self.update;
        app.run(move || loop {
                    let message = view.stream().pop();
                    model = update.update(&model, message);
                    view.update(&model);
                })
    }
}
