
use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};

use cocoa::appkit::{NSWindow, NSBackingStoreBuffered};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use cocoa::appkit::{NSTitledWindowMask, NSMiniaturizableWindowMask, NSResizableWindowMask,
                    NSClosableWindowMask};

use super::id::{Id, AtomicId};
use super::widget::Widget;

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

pub struct Window {
    _window: AtomicId,
}

pub struct Stack {
    pub id: Id,
}

impl<S> Widget<S> for Stack {
    fn id(&self) -> &Id {
        &self.id
    }

    fn add(&mut self, widget: &Box<Widget<S>>) {
        unsafe {
            msg_send![*self.id, addView:**widget.id()
                              inGravity:NSStackViewGravity::Top];
        };
    }
}

impl Stack {
    pub fn new() -> Self {
        unsafe {
            let stack = {
                let stack: id = msg_send![class("NSStackView"), alloc];
                let stack: id = msg_send![stack, init];

                // TODO: window.frame padded by 10.0 on each side?
                let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
                msg_send![stack, setFrame: rect];

                msg_send![stack, setAutoresizingMask: NSViewWidthSizable | NSViewHeightSizable];

                msg_send![stack, setOrientation: UserInterfaceLayoutOrientation::Vertical];
                msg_send![stack, setSpacing: 5.0];

                let insets = NSEdgeInsets::new(10., 10., 10., 10.);
                msg_send![stack, setEdgeInsets: insets];

                stack
            };

            Stack { id: stack.into() }
        }
    }
}

impl Window {
    pub fn new(title: &str) -> (Self, Stack) {
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

            let stack = Stack::new();
            msg_send![window.contentView(), addSubview:stack.id.clone()];

            (Window { _window: window.into() }, stack)
        }
    }
}
