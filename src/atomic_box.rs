
use std;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicBox<T> {
    pub fn new(value: T) -> AtomicBox<T> {
        AtomicBox { ptr: AtomicPtr::new(Box::into_raw(Box::new(value))) }
    }
}

impl<T> std::ops::Deref for AtomicBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.ptr.load(Ordering::Relaxed);
        unsafe { &*ptr }
    }
}

impl<T> std::ops::DerefMut for AtomicBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
