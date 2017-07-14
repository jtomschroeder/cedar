
use super::id::Id;
use dom::Attributes;

pub trait Widget {
    fn id(&self) -> &Id;

    fn update(&mut self, Attributes) {}
    fn add(&mut self, Box<Widget>) {}
}
