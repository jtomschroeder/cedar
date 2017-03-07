
use std::sync::Arc;

use super::widget::Widget;
use atomic_box::AtomicBox;

use gtk;
use gtk::prelude::*;

pub struct Window<M> {
    vbox: gtk::Box,
    _window: gtk::Window,
    views: Arc<Vec<AtomicBox<Box<Widget<M>>>>>,
}

impl<M> Window<M> {
    pub fn new(title: &str) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        window.set_title(title);
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 20);
        window.add(&vbox);

        window.show_all();
        // window.present();

        Window {
            vbox: vbox,
            _window: window,
            views: Arc::new(Vec::new()),
        }
    }

    pub fn add<V: Widget<M> + 'static>(&mut self, view: V) {
        view.add(&self.vbox);

        if let Some(views) = Arc::get_mut(&mut self.views) {
            views.push(AtomicBox::new(Box::new(view)));
        }
    }

    pub fn update(&mut self, model: &M) {
        if let Some(views) = Arc::get_mut(&mut self.views) {
            for view in views.iter_mut() {
                view.update(model);
            }
        }
    }
}
