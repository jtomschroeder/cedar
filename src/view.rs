
use super::cacao;
use stream::Stream;

pub struct View<M> {
    window: cacao::Window,
    stream: Stream<M>,
}

impl<M: 'static> View<M> {
    pub fn new() -> Self {
        View {
            window: cacao::Window::new("buttons"),
            stream: Stream::new(),
        }
    }

    pub fn update(&mut self, model: i32) {
        self.window.update(model)
    }

    pub fn stream(&mut self) -> &Stream<M> {
        &self.stream
    }

    pub fn button<F>(mut self, f: F) -> Self
        where F: FnOnce(cacao::Button<M>) -> cacao::Button<M>
    {
        let button = f(cacao::Button::new(self.stream.clone()));
        self.window.add(button);
        self
    }

    pub fn label<F>(mut self, f: F) -> Self
        where F: FnOnce(cacao::Label) -> cacao::Label
    {
        let label = f(cacao::Label::new());
        self.window.add(label);
        self
    }
}
