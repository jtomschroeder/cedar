
use objc::runtime::{Object, Sel, Class};
use objc::declare::ClassDecl;

use cocoa::base::id;
use cocoa::base::class;

use std;
type Void = std::os::raw::c_void;

pub trait Actionable {
    fn act(&mut self);
}

impl<F> Actionable for F
    where F: FnMut()
{
    fn act(&mut self) {
        self()
    }
}

struct Action(pub Box<Actionable>);

impl Action {
    fn act(&mut self) {
        self.0.act();
    }
}

pub fn register() {
    let superclass = Class::get("NSObject").unwrap();

    let mut decl = match ClassDecl::new("Action", superclass) {
        Some(decl) => decl,
        _ => return, // already registered!
    };

    decl.add_ivar::<*mut Void>("_action");

    extern "C" fn action_initialize(this: &mut Object, _: Sel, action: *mut Void) {
        unsafe { this.set_ivar("_action", action) };
    }

    extern "C" fn action_act(this: &mut Object, _cmd: Sel) {
        unsafe {
            let action: &mut Box<Action> =
                std::mem::transmute(this.get_mut_ivar::<*mut Void>("_action"));
            action.act();
        }
    }

    extern "C" fn action_dealloc(this: &Object, _cmd: Sel) {
        // => [super dealloc];
        if let Some(superclass) = this.class().superclass() {
            unsafe { msg_send![super(this, superclass), dealloc] };
        }
    }

    unsafe {
        decl.add_method(sel!(initialize:),
                        action_initialize as extern "C" fn(&mut Object, Sel, *mut Void));

        decl.add_method(sel!(act), action_act as extern "C" fn(&mut Object, Sel));
        decl.add_method(sel!(dealloc), action_dealloc as extern "C" fn(&Object, Sel));
    }

    decl.register();
}

pub fn create<F: FnMut() + 'static>(action: F) -> id {
    register();

    unsafe {
        // TODO: handle `release` of Action

        let act: id = msg_send![class("Action"), alloc];
        let target: id = msg_send![act, init];

        let action = Box::new(Action(Box::new(action)));

        let action: *mut Void = std::mem::transmute(Box::into_raw(action));
        msg_send![target, initialize: action];

        act
    }
}
