
extern crate futures;
extern crate crossbeam;

use std::sync::Arc;
use std::thread;

use crossbeam::sync::MsQueue;

pub struct Queue<T> {
    queue: MsQueue<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: MsQueue::new() }
    }

    // streaming!
    pub fn roll<U>(&self, init: U, app: fn(U, T) -> U) {
        let mut acc = init;
        loop {
            acc = app(acc, self.queue.pop());
        }
    }
}

pub type Reducer<M, S> = fn(M, S) -> M;

// Middleware?
pub type Watcher<M, S> = fn(&S, &M);

// pub struct Flux<M, S> {}

type Sink<S> = Arc<Queue<S>>;

// trait Subscription {}

// trait Task {}

pub fn flux<M, S>(model: M, reducer: Reducer<M, S>) -> Sink<S>
    where M: Sync + Send + 'static,
          S: Sync + Send + 'static
{
    let queue = Arc::new(Queue::new());

    {
        let stream = queue.clone();
        let _ = thread::spawn(move || stream.roll(model, reducer));
    }

    queue
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
