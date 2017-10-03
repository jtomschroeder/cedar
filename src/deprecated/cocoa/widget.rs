
use super::id::Id;
use dom::Attributes;

pub trait Widget<S> {
    fn id(&self) -> &Id;

    fn update(&mut self, Attributes<S>) {}

    // TODO: once layout engine is integrated, remove `add` method
    // - 'children' won't *actually* be added to widgets

    fn add(&mut self, &Box<Widget<S>>) {}

    fn layout(&mut self, left: f64, top: f64, width: f64, height: f64) {
        use cocoa::foundation::{NSRect, NSPoint, NSSize};

        let rect = NSRect::new(NSPoint::new(left, top), NSSize::new(width, height));

        let widget = self.id();
        unsafe { msg_send![**widget, setFrame: rect] };
    }
}
