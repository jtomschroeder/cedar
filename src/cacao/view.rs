
use cocoa::base::id;

pub trait View {
    fn id(&self) -> id;

    fn update(&mut self, _model: i32) {}
}
