
use objc::runtime::{Object, Sel, Class};
use objc::declare::ClassDecl;

use cocoa::base::id;
use cocoa::base::class;

use std;
type Void = std::os::raw::c_void;

pub trait Delegatable {
    fn delegate(&mut self, text: &str);
}

impl<F> Delegatable for F
    where F: FnMut(&str)
{
    fn delegate(&mut self, text: &str) {
        self(text)
    }
}

struct Delegate(pub Box<Delegatable>);

impl Delegate {
    fn delegate(&mut self, text: &str) {
        self.0.delegate(text);
    }
}

pub fn register<F>(class: &str, mut f: F)
    where F: FnMut(&mut ClassDecl)
{
    let superclass = Class::get("NSObject").expect("NSObject");

    let mut decl = match ClassDecl::new(class, superclass) {
        Some(decl) => decl,
        _ => return, // already registered!
    };

    decl.add_ivar::<*mut Void>("_delegate");

    extern "C" fn delegate_initialize(this: &mut Object, _: Sel, delegate: *mut Void) {
        unsafe { this.set_ivar("_delegate", delegate) };
    }

    extern "C" fn delegate_dealloc(this: &Object, _cmd: Sel) {
        // => [super dealloc];
        if let Some(superclass) = this.class().superclass() {
            unsafe { msg_send![super(this, superclass), dealloc] };
        }
    }

    unsafe {
        decl.add_method(sel!(initialize:),
                        delegate_initialize as extern "C" fn(&mut Object, Sel, *mut Void));

        f(&mut decl);

        decl.add_method(sel!(dealloc),
                        delegate_dealloc as extern "C" fn(&Object, Sel));
    }

    decl.register();
}

pub fn create<F: FnMut(&str) + 'static>(delegate: F) -> id {
    register("CRTextDelegate", |decl| {
        extern "C" fn delegate_act(this: &mut Object, _cmd: Sel, notification: id) {
            use cocoa::foundation::NSString;

            unsafe {
                let field: id = msg_send![notification, object];
                let text: id = msg_send![field, stringValue];

                let string = std::slice::from_raw_parts(NSString::UTF8String(text),
                                                        NSString::len(text));

                let string = std::str::from_utf8(std::mem::transmute::<&[i8], &[u8]>(string));

                let delegate: &mut Box<Delegate> =
                    std::mem::transmute(this.get_mut_ivar::<*mut Void>("_delegate"));
                delegate.delegate(string.unwrap());
            }
        }

        unsafe {
            decl.add_method(sel!(controlTextDidChange:),
                            delegate_act as extern "C" fn(&mut Object, Sel, id))
        };
    });

    unsafe {
        // TODO: handle `release` of Delegate

        let act: id = msg_send![class("CRTextDelegate"), alloc];
        let target: id = msg_send![act, init];

        let delegate = Box::new(Delegate(Box::new(delegate)));

        let delegate: *mut Void = std::mem::transmute(Box::into_raw(delegate));
        msg_send![target, initialize: delegate];

        act
    }
}
