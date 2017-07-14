
use super::id::Id;

pub trait Widget {
    fn id(&self) -> &Id;

    // fn update(&mut self, model: &M);

    fn add(&mut self, widget: Box<Widget>) {}
}
