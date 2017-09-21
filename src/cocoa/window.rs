
#![allow(non_upper_case_globals)]

use cocoa::base::{nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};

use cocoa::appkit::{NSWindow, NSView, NSBackingStoreBuffered};
use cocoa::appkit::{NSTitledWindowMask, NSMiniaturizableWindowMask, NSResizableWindowMask,
                    NSClosableWindowMask};

use super::id::{Id, AtomicId};
use super::widget::Widget;

pub struct Window {
    _window: AtomicId,
}

pub struct Container {
    pub id: Id,
}

impl<S> Widget<S> for Container {
    fn id(&self) -> &Id {
        &self.id
    }

    fn add(&mut self, widget: &Box<Widget<S>>) {
        unsafe { NSView::addSubview_(*self.id, **widget.id()) };
    }
}

impl Container {
    pub fn new() -> Self {
        let frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(500., 500.));

        let view = unsafe { NSView::alloc(nil).initWithFrame_(frame).autorelease() };

        Container { id: view.into() }
    }
}

impl Window {
    pub fn new(title: &str) -> (Self, Container) {
        let style = NSResizableWindowMask | NSTitledWindowMask | NSMiniaturizableWindowMask |
            NSClosableWindowMask;

        let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(500., 500.));

        let window = unsafe {
            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    rect,
                    style,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease();

            window.cascadeTopLeftFromPoint_(NSPoint::new(0., 0.));
            window.center();

            let title = NSString::alloc(nil).init_str(title);
            window.setTitle_(title);

            window.makeKeyAndOrderFront_(nil);

            window
        };

        (
            Window { _window: window.into() },
            Container { id: unsafe { window.contentView() }.into() },
        )
    }
}
