
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

    fn layout(&mut self, top: f64, left: f64, width: f64, height: f64) {
        use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};
        // let rect = NSRect::new(NSPoint::new(100., 100.), NSSize::new(100., 100.));
        // let button: id = msg_send![button, initWithFrame: rect];

        let widget = self.id();

        println!("Setting frame!");

        let rect = NSRect::new(NSPoint::new(left, top), NSSize::new(width, height));
        unsafe { msg_send![**widget, setFrame: rect] };
    }
}
