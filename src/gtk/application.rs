
use gtk;

pub struct Application;

impl Application {
    pub fn new() -> Self {
        gtk::init().unwrap();

        Application {}
    }

    pub fn run<F: FnMut() + Send + 'static>(self, mut action: F) {
        gtk::timeout_add(16, move || {
            action();
            gtk::Continue(true)
        });

        gtk::main()
    }
}

impl Drop for Application {
    fn drop(&mut self) {}
}
