
use super::id::Id;

pub trait Widget<M> {
    fn id(&self) -> &Id;

    fn update(&mut self, model: &M);
}
