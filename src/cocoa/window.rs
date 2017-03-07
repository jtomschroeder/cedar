
use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};

use cocoa::appkit::{NSWindow, NSBackingStoreBuffered};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use cocoa::appkit::{NSTitledWindowMask, NSMiniaturizableWindowMask, NSResizableWindowMask,
                    NSClosableWindowMask};

use std::sync::Arc;

use super::id::AtomicId;
use super::widget::Widget;
use atomic_box::AtomicBox;

#[repr(u64)]
enum UserInterfaceLayoutOrientation {
    Vertical = 1,
}

#[repr(u64)]
enum NSStackViewGravity {
    Top = 1,
}

use core_graphics::base::CGFloat;

#[repr(C)]
pub struct NSEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

impl NSEdgeInsets {
    fn new(top: CGFloat, left: CGFloat, bottom: CGFloat, right: CGFloat) -> Self {
        NSEdgeInsets {
            top: top,
            left: left,
            bottom: bottom,
            right: right,
        }
    }
}

pub struct Window<M> {
    window: AtomicId,
    stack: AtomicId,
    views: Arc<Vec<AtomicBox<Box<Widget<M>>>>>,
}

impl<M> Window<M> {
    pub fn new(title: &str) -> Self {
        unsafe {
            let style = NSResizableWindowMask as NSUInteger | NSTitledWindowMask as NSUInteger |
                        NSMiniaturizableWindowMask as NSUInteger |
                        NSClosableWindowMask as NSUInteger;
            let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(rect,
                                                              style,
                                                              NSBackingStoreBuffered,
                                                              NO)
                .autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(0., 0.));
            window.center();

            let title = NSString::alloc(nil).init_str(title);
            window.setTitle_(title);

            window.makeKeyAndOrderFront_(nil);

            let stack = {
                let stack: id = msg_send![class("NSStackView"), alloc];
                let stack: id = msg_send![stack, init];

                // window.frame padded by 10.0 on each side
                let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
                msg_send![stack, setFrame: rect];

                msg_send![stack, setAutoresizingMask: NSViewWidthSizable | NSViewHeightSizable];

                msg_send![stack, setOrientation: UserInterfaceLayoutOrientation::Vertical];
                msg_send![stack, setSpacing: 5.0];

                let insets = NSEdgeInsets::new(10., 10., 10., 10.);
                msg_send![stack, setEdgeInsets: insets];

                use cocoa::appkit::NSView;
                window.contentView().addSubview_(stack);

                stack
            };

            Window {
                window: window.into(),
                stack: stack.into(),
                views: Arc::new(Vec::new()),
            }
        }
    }

    pub fn add<V: Widget<M> + 'static>(&mut self, view: V) {
        unsafe {
            msg_send![self.stack.get(), addView:**view.id()
                                      inGravity:NSStackViewGravity::Top];

            msg_send![self.window.get(), layoutIfNeeded];

            let frame: NSRect = msg_send![self.stack.get(), frame];
            msg_send![self.window.get(), setContentSize:frame.size];
        };

        if let Some(views) = Arc::get_mut(&mut self.views) {
            views.push(AtomicBox::new(Box::new(view)));
        }
    }

    pub fn update(&mut self, model: &M) {
        if let Some(views) = Arc::get_mut(&mut self.views) {
            for view in views.iter_mut() {
                view.update(model);
            }
        }
    }
}
