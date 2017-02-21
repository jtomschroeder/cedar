
use super::cacao;

pub struct View {
    window: cacao::Window,
}

impl View {
    pub fn new() -> Self {
        View { window: cacao::Window::new("buttons") }
    }

    pub fn update(&mut self, model: i32) {
        self.window.update(model)
    }

    pub fn button<F>(mut self, f: F) -> Self
        where F: FnOnce(cacao::Button) -> cacao::Button
    {
        let button = f(cacao::Button::new());
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
