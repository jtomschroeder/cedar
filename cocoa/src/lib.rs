
mod bindings {
    use super::*;

    extern "C" {
        pub fn run(ic: *mut Interconnect);
    }
}

pub struct Interconnect;

impl Interconnect {
    fn send(&self) {
        eprintln!("HEY!");
    }
}

#[no_mangle]
pub extern "C" fn ic_send(ic: *mut Interconnect) {
    let ic: &Interconnect = unsafe { &*ic };
    ic.send();
}

pub fn run() {
    let interconnect = Box::new(Interconnect);
    unsafe { bindings::run(Box::into_raw(interconnect)) }
}
