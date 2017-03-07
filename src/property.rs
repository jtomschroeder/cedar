
pub trait Property<M, T> {
    fn process(&mut self, model: &M) -> T;
}

impl<M, T, F> Property<M, T> for F
    where F: Fn(&M) -> T
{
    fn process(&mut self, model: &M) -> T {
        self(model)
    }
}

impl<M> Property<M, String> for String {
    fn process(&mut self, _: &M) -> String {
        self.clone()
    }
}

impl<M> Property<M, String> for &'static str {
    fn process(&mut self, _: &M) -> String {
        self.to_string()
    }
}
