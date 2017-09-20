
#![allow(non_upper_case_globals)]

use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};

use cocoa::appkit::{NSWindow, NSBackingStoreBuffered};
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
        unsafe { msg_send![*self.id, addSubview:**widget.id()] };
    }
}

impl Container {
    pub fn new() -> Self {
        unsafe {
            let view = {
                let view: id = msg_send![class("NSView"), alloc];

                let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(500., 500.));
                let view: id = msg_send![view, initWithFrame:rect];

                view
            };

            Container { id: view.into() }
        }
    }
}

impl Window {
    pub fn new(title: &str) -> (Self, Container) {
        unsafe {
            let style = NSResizableWindowMask | NSTitledWindowMask | NSMiniaturizableWindowMask |
                NSClosableWindowMask;
            let rect = NSRect::new(NSPoint::new(500., 500.), NSSize::new(500., 500.));
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

            (
                Window { _window: window.into() },
                Container { id: window.contentView().into() },
            )
        }
    }
}
