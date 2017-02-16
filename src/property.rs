
pub trait Property<T> {
    fn process(&mut self, model: i32) -> T;
}

impl<T, F> Property<T> for F
    where F: FnMut(i32) -> T
{
    fn process(&mut self, model: i32) -> T {
        self(model)
    }
}

impl Property<String> for String {
    fn process(&mut self, _: i32) -> String {
        self.clone()
    }
}

impl Property<String> for &'static str {
    fn process(&mut self, _: i32) -> String {
        self.to_string()
    }
}
