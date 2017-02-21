
use cocoa::base::id;

pub trait Widget {
    fn id(&self) -> id;

    fn update(&mut self, _model: i32) {}
}
