
use std;
use std::sync::atomic::{AtomicPtr, Ordering};

use objc;
use cocoa::base::id;

pub struct Id(id);

impl Id {
    pub fn new(id: id) -> Self {
        Id(id)
    }
}

impl std::ops::Deref for Id {
    type Target = id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<id> for Id {
    fn from(id: id) -> Self {
        Self::new(id)
    }
}

impl Drop for Id {
    fn drop(&mut self) {
        unsafe { msg_send![self.0, release] };
    }
}

pub struct AtomicId(AtomicPtr<objc::runtime::Object>);

impl AtomicId {
    pub fn new(id: id) -> Self {
        AtomicId(AtomicPtr::new(id))
    }

    pub fn load(&self) -> id {
        self.0.load(Ordering::Relaxed)
    }

    pub fn get(&mut self) -> id {
        *self.0.get_mut()
    }
}

impl From<id> for AtomicId {
    fn from(id: id) -> Self {
        Self::new(id)
    }
}

impl Drop for AtomicId {
    fn drop(&mut self) {
        unsafe { msg_send![self.get(), release] };
    }
}
