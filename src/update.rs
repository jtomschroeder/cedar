
pub trait Update<M, S> {
    fn update(&mut self, model: M, message: S) -> M;
}

impl<M, S, F> Update<M, S> for F
    where F: FnMut(M, S) -> M
{
    fn update(&mut self, model: M, message: S) -> M {
        self(model, message)
    }
}
