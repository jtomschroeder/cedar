
use objc;

use cocoa::base::{nil, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};
use cocoa::appkit::{NSWindow, NSTitledWindowMask, NSBackingStoreBuffered};

use std::sync::atomic::AtomicPtr;
use std::sync::Arc;

use cacao::view::View;
use atomic_box::AtomicBox;

pub struct Window {
    id: AtomicPtr<objc::runtime::Object>,
    views: Arc<Vec<AtomicBox<Box<View>>>>,
}

impl Window {
    pub fn new(title: &str) -> Self {
        unsafe {
            let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(350., 350.));
            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(rect,
                                                              NSTitledWindowMask as NSUInteger,
                                                              NSBackingStoreBuffered,
                                                              NO)
                .autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();

            let title = NSString::alloc(nil).init_str(title);
            window.setTitle_(title);

            window.makeKeyAndOrderFront_(nil);

            Window {
                id: AtomicPtr::new(window),
                views: Arc::new(Vec::new()),
            }
        }
    }

    pub fn add<V: View + 'static>(&mut self, view: V) {
        use cocoa::appkit::NSView;
        unsafe { self.id.get_mut().contentView().addSubview_(view.view()) };

        if let Some(views) = Arc::get_mut(&mut self.views) {
            views.push(AtomicBox::new(Box::new(Box::new(view))));
        }
    }

    pub fn update(&mut self, model: i32) {
        if let Some(views) = Arc::get_mut(&mut self.views) {
            for view in views.iter_mut() {
                view.get_mut().update(model);
            }
        }
    }
}
