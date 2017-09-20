
use super::id::Id;
use dom::Attributes;

pub trait Widget<S> {
    fn id(&self) -> &Id;

    fn update(&mut self, Attributes<S>) {}

    // TODO: once layout engine is integrated, remove `add` method
    // - 'children' won't *actually* be added to widgets

    fn add(&mut self, &Box<Widget<S>>) {}

    // TODO: set size
    // TODO: set position
}
