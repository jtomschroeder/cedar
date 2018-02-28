use std::ffi::CString;

pub trait Processor {
    fn process(&mut self, s: String);
}

static mut PROCESSOR: Option<Box<Processor>> = None;

#[no_mangle]
pub extern "C" fn process(s: *mut i8) {
    unsafe {
        let s = CString::from_raw(s);
        let s = s.into_string().unwrap();

        if let Some(ref mut processor) = PROCESSOR {
            processor.process(s);
        }
    }
}

pub fn initialize<P: Processor + 'static>(processor: P) {
    unsafe { PROCESSOR = Some(Box::new(processor)) };
}
