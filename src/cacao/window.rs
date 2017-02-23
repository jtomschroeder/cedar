
use objc;

use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};
use cocoa::appkit::{NSWindow, NSTitledWindowMask, NSBackingStoreBuffered};

use std::sync::atomic::AtomicPtr;
use std::sync::Arc;

use cacao::view::View;
use atomic_box::AtomicBox;

#[repr(u64)]
enum UserInterfaceLayoutOrientation {
    Vertical = 1,
}

pub struct Window<M> {
    _id: AtomicPtr<objc::runtime::Object>,
    stack: AtomicPtr<objc::runtime::Object>,
    views: Arc<Vec<AtomicBox<Box<View<M>>>>>,
}

impl<M: Clone> Window<M> {
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

            let stack = {
                let stack: id = msg_send![class("NSStackView"), alloc];
                let stack: id = msg_send![stack, init];

                // window.frame padded by 10.0 on each side
                let rect = NSRect::new(NSPoint::new(10., 10.), NSSize::new(330., 330.));
                msg_send![stack, setFrame: rect];

                msg_send![stack, setOrientation: UserInterfaceLayoutOrientation::Vertical];
                msg_send![stack, setSpacing: 25.0];

                use cocoa::appkit::NSView;
                window.contentView().addSubview_(stack);

                stack
            };

            Window {
                _id: AtomicPtr::new(window),
                stack: AtomicPtr::new(stack),
                views: Arc::new(Vec::new()),
            }
        }
    }

    pub fn add<V: View<M> + 'static>(&mut self, view: V) {
        unsafe { msg_send![*self.stack.get_mut(), addArrangedSubview: view.id()] };

        if let Some(views) = Arc::get_mut(&mut self.views) {
            views.push(AtomicBox::new(Box::new(Box::new(view))));
        }
    }

    pub fn update(&mut self, model: M) {
        if let Some(views) = Arc::get_mut(&mut self.views) {
            for view in views.iter_mut() {
                view.get_mut().update(model.clone());
            }
        }
    }
}
