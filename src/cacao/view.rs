
use cocoa::base::id;

pub trait View<M> {
    fn id(&self) -> id;

    fn update(&mut self, model: &M);
}
