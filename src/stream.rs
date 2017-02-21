
use std;
use std::sync::Arc;
use crossbeam::sync::MsQueue;

pub struct Stream<T>(Arc<MsQueue<T>>);

impl<T> Stream<T> {
    pub fn new() -> Self {
        Stream(Arc::new(MsQueue::new()))
    }
}

impl<T> Clone for Stream<T> {
    fn clone(&self) -> Self {
        Stream(self.0.clone())
    }
}

impl<T> std::ops::Deref for Stream<T> {
    type Target = Arc<MsQueue<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
