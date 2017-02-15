
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicBox<T> {
    pub fn new(value: Box<T>) -> AtomicBox<T> {
        AtomicBox { ptr: AtomicPtr::new(Box::into_raw(value)) }
    }

    pub fn get_mut(&mut self) -> &mut T {
        let ptr = self.ptr.load(Ordering::Relaxed);
        unsafe { &mut *ptr }
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Acquire);
        unsafe { Box::from_raw(ptr) };
    }
}
