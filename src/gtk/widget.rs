
use gtk;

pub trait Widget<M> {
    fn add(&self, container: &gtk::Box);

    fn update(&mut self, model: &M);
}
