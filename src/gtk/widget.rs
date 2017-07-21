
use gtk;

// pub trait Widget<M> {
//     fn add(&self, container: &gtk::Box);

//     fn update(&mut self, model: &M);
// }

use dom::Attributes;

pub trait Widget<S> {
    // fn id(&self) -> &Id;

    fn update(&mut self, Attributes<S>) {}

    // fn add(&self, container: &gtk::Box);

    fn add(&self, &Box<Widget<S>>) {}
    fn add_in(&self, &Widget<S>) {}
}

#[derive(Debug)]
pub enum NWidget {
    Button,
    Stack,
    Label,
}
