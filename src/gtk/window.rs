
use super::widget::{Widget, NWidget};

use gtk;
use gtk::prelude::*;

pub struct Stack {
    pub stack: gtk::Box,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: gtk::Box::new(gtk::Orientation::Vertical, 20) }
    }

    pub fn from(stack: gtk::Box) -> Self {
        Stack { stack }
    }

    pub fn add<S>(&self, widget: &NWidget<S>) {
        match widget {
            &NWidget::Button(ref button) => {
                self.stack.add(&button.button);
                button.button.show();
            }
            &NWidget::Stack(ref stack) => {
                self.stack.add(&stack.stack);
                stack.stack.show();
            }
            &NWidget::Label(ref label) => {
                self.stack.add(&label.label);
                label.label.show();
            }
            &NWidget::Field(ref field) => {
                self.stack.add(&field.entry);
                field.entry.show();
            }
        }
    }
}

impl<S> Widget<S> for Stack {}

pub struct Window {
    _window: gtk::Window,
}

impl Window {
    pub fn new(title: &str) -> (Self, Stack) {
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

        (Window { _window: window }, Stack::from(vbox))
    }
}
