
extern crate gtk;

use std::sync::Arc;
use std::collections::HashMap;

use self::gtk::prelude::*;
use self::gtk::{Button, Window, WindowType};

use crossbeam::sync::MsQueue;

use renderer::{self, Command, Event};

#[derive(Clone)]
pub struct Renderer {
    pub commands: Arc<MsQueue<Command>>,
    pub events: Arc<MsQueue<Event>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            commands: Arc::new(MsQueue::new()),
            events: Arc::new(MsQueue::new()),
        }
    }
}

impl renderer::Renderer for Renderer {
    fn send(&self, cmd: Command) {
        self.commands.push(cmd)
    }

    fn recv(&self) -> Event {
        self.events.pop()
    }
}

pub fn run(renderer: Renderer) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("** cedar **");
    window.set_default_size(500, 500);

    window.connect_delete_event(|_, _| {
        println!("Quit!");

        gtk::main_quit();
        Inhibit(false)
    });

    let mut widgets = HashMap::new();

    gtk::timeout_add(16, move || {
        if let Some(command) = renderer.commands.try_pop() {
            println!("Command: {:?}", command);

            // TODO: handle commands
            // - create
            // - update
            // - move
        }

        if widgets.is_empty() {
            println!("Adding button!");

            let button = Button::new_with_label("Click me!");
            window.add(&button);

            button.connect_clicked(|_| {
                println!("Clicked!");
            });

            widgets.insert("1", button);

            window.show_all();
        }

        gtk::Continue(true)
    });

    gtk::main();
}
