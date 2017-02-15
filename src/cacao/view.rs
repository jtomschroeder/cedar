
use cocoa::base::id;

pub trait View {
    fn view(&self) -> id;

    fn update(&mut self, _model: i32) {}
}
