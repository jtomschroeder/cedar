
extern crate glib;
extern crate gtk;

use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::cell::RefCell;

use self::gtk::prelude::*;
use self::gtk::{Button, Window, WindowType, Orientation, Label};

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

enum Widget {
    Button(Button),
    Label(Label),
    Field(gtk::Entry),
}


struct Updater {
    widgets: HashMap<String, Widget>,

    window: Window,
    container: gtk::Box,

    renderer: Renderer,
}

impl Updater {
    fn new(window: Window, container: gtk::Box, renderer: Renderer) -> Self {
        Updater {
            widgets: HashMap::new(),
            window,
            container,
            renderer,
        }
    }

    fn update(&mut self, command: Command) {
        match command {
            Command::Create {
                id,
                kind,
                attributes,
            } => {
                match kind.as_str() {
                    "Button" => {
                        let button = Button::new_with_label(&attributes["Text"]);
                        self.container.add(&button);

                        {
                            let id = id.clone();
                            let events = self.renderer.events.clone();
                            button.connect_clicked(
                                move |_| events.push(Event::Click { id: id.clone() }),
                            );
                        }

                        self.widgets.insert(id, Widget::Button(button));
                    }

                    "Label" => {
                        let label = Label::new(Some(attributes["Text"].as_str()));
                        self.container.add(&label);

                        self.widgets.insert(id, Widget::Label(label));
                    }

                    "Field" => {
                        let field = gtk::Entry::new();
                        self.container.add(&field);

                        if let Some(ref placeholder) = attributes.get("Placeholder") {
                            field.set_placeholder_text(Some(placeholder.as_str()))
                        }

                        {
                            let id = id.clone();
                            let events = self.renderer.events.clone();
                            field.connect_event(move |field, _| {
                                if let Some(ref text) = field.get_text() {
                                    events.push(Event::Change {
                                        id: id.clone(),
                                        value: text.clone(),
                                    });
                                }

                                gtk::Inhibit(false)
                            });
                        }

                        self.widgets.insert(id, Widget::Field(field));
                    }

                    _ => unimplemented!(),
                }
            }

            Command::Update(id, attribute, value) => {
                let ref widget = self.widgets[&id];
                match widget {
                    &Widget::Label(ref label) if attribute == "Text" => label.set_label(&value),

                    &Widget::Field(ref field) if attribute == "Placeholder" => {
                        field.set_placeholder_text(Some(value.as_str()))
                    }

                    _ => unimplemented!(),
                }
            }

            Command::Remove(id) => {
                if let Some(widget) = self.widgets.remove(&id) {
                    match widget {
                        Widget::Button(button) => button.destroy(),
                        Widget::Label(label) => label.destroy(),
                        Widget::Field(field) => field.destroy(),
                    }
                }
            }
        }

        self.window.show_all();
    }
}

thread_local!(
    static GLOBAL: RefCell<Option<(Arc<MsQueue<Command>>, Updater)>> = RefCell::new(None)
);

pub fn run(renderer: Renderer) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("** cedar **");
    window.set_default_size(500, 500);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let container = gtk::Box::new(Orientation::Vertical, 5);
    window.add(&container);

    let commands = Arc::new(MsQueue::new());

    {
        let renderer = renderer.clone();
        let commands = commands.clone();
        GLOBAL.with(move |global| {
            *global.borrow_mut() = Some((commands, Updater::new(window, container, renderer)))
        });
    }

    thread::spawn(move || loop {
        // Push command to 'updater' to run on main (idle) thread
        commands.push(renderer.commands.pop());

        // main thread!
        glib::idle_add(|| {
            GLOBAL.with(|global| if let Some((ref mut commands,
                                  ref mut updater)) =
                *global.borrow_mut()
            {
                updater.update(commands.pop());
            });

            glib::Continue(false)
        });
    });

    gtk::main();
}
