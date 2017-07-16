
use super::id::Id;
use super::Attributes;

pub trait Widget<S> {
    fn id(&self) -> &Id;

    fn update(&mut self, Attributes<S>) {}

    fn add(&mut self, &Box<Widget<S>>) {}
}
