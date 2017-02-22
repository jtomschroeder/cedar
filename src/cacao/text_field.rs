
use cocoa::base::{id, nil, class};
use cocoa::foundation::NSString;

use cacao::view::View;
use cacao::delegate;

use property::Property;
use stream::Stream;

pub struct TextField<S> {
    id: id,
    stream: Stream<S>,
}

impl<S: 'static> TextField<S> {
    pub fn new(stream: Stream<S>) -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("");
            let label: id = msg_send![class("NSTextField"), textFieldWithString: string];

            TextField {
                id: label,
                stream: stream,
            }
        }
    }

    pub fn position(self, x: f64, y: f64) -> Self {
        use cocoa::foundation::NSRect;

        let mut frame: NSRect = unsafe { msg_send![self.id, frame] };
        frame.origin.x = x;
        frame.origin.y = y;
        unsafe { msg_send![self.id, setFrame: frame] };

        self
    }

    pub fn placeholder(self, text: &str) -> Self {
        unsafe {
            let text = NSString::alloc(nil).init_str(text);

            let string: id = msg_send![class("NSAttributedString"), alloc];
            let string: id = msg_send![string, initWithString: text];

            msg_send![self.id, setPlaceholderAttributedString: string];
            msg_send![self.id, sizeToFit];
        }

        self
    }

    pub fn change<F: FnMut(&str) -> S + 'static>(self, mut delegate: F) -> Self {
        let stream = self.stream.clone();
        let delegate = delegate::create(move |s| stream.push(delegate(s)));

        unsafe { msg_send![self.id, setDelegate: delegate] };

        self
    }
}

impl<M, S: 'static> View<M> for TextField<S> {
    fn id(&self) -> id {
        self.id
    }

    fn update(&mut self, _: M) {}
}
