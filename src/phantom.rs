
use std::str;
use std::collections::HashMap;

use dom;
use tree::{self, Vertex};
use renderer::{Command, Event};
use program::{View, Action};

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn commands<T: Clone>(dom: &dom::Object<T>, set: dom::Changeset) -> Vec<Command> {
    let mut commands = vec![];

    fn expand<S>(root: &tree::Path, node: &dom::Object<S>, commands: &mut Vec<Command>) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.traverse(root, |path, node| {
            // eprintln!("id: {:?} {:?}", root, path);

            let id = path.to_string();

            let mut attributes = HashMap::new();

            // let kind = match node.widget {
            //     dom::Widget::Div => Some("Div".into()),

            //     dom::Widget::Text(ref text) => {
            //         attributes.insert("Text".into(), text.text.clone());
            //         Some("Text".into())
            //     }

            //     dom::Widget::Button(ref button) => {
            //         attributes.insert("Text".into(), button.text.clone());
            //         Some("Button".into())
            //     }

            //     dom::Widget::Input(ref input) => {
            //         if let Some(ref placeholder) = input.placeholder {
            //             attributes.insert("Placeholder".into(), placeholder.clone());
            //         }
            //         Some("Input".into())
            //     }
            // };

            let kind = node.widget.element.to_string();
            let value = node.widget.value.clone();

            if let Some(ref placeholder) = node.widget.placeholder {
                attributes.insert("placeholder".into(), placeholder.clone());
            }

            let parent = path.parent().to_string();
            commands.push(Command::Create {
                id,
                parent,
                kind,
                value,
                attributes,
            })
        });
    }

    for (path, op) in set.into_iter() {
        let node = || dom.find(&path).expect("path in nodes");
        let id = || path.to_string();

        match op {
            tree::Operation::Create => expand(&path, node(), &mut commands),
            tree::Operation::Update => {
                let node = node();
                match node.widget.element {
                    dom::Element::Text => {
                        commands.push(Command::Update {
                            id: id(),
                            attribute: "Text".into(),
                            value: node.widget.value.clone().unwrap(),
                        })
                    }

                    _ => panic!("`Update` not yet implemented for widget!"),
                }
            }

            tree::Operation::Delete => commands.push(Command::Remove { id: id() }),

            tree::Operation::Replace => panic!("`Replace` not yet implemented!"),
        }
    }

    commands
}

pub struct Phantom<S> {
    dom: dom::Object<S>,
}

impl<S> Phantom<S>
where
    S: 'static + Clone + Send + PartialEq,
{
    pub fn initialize<M>(model: &M, view: View<M, S>) -> (Self, Vec<Command>) {
        let dom = view(&model);

        // Create changeset: Create @ 'root'
        let patch = vec![(tree::Path::new(), tree::Operation::Create)];

        let commands = commands(&dom, patch);

        (Phantom { dom }, commands)
    }

    pub fn translate(&self, event: Event) -> Option<Action<S>> {

        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        let path =
            |id: &str| tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());

        let ref dom = self.dom;
        match event {
            Event::Click { id } => {
                let path = path(&id);
                dom.find(&path).and_then(|node| {
                    node.widget.click.clone().map(Action::Update)
                })
            }

            Event::Change { id, value } => {
                let path = path(&id);
                dom.find(&path).and_then(|node| {
                    node.widget.change.map(|c| c(value)).map(Action::Update)
                })
            }
        }
    }

    pub fn update<M>(&mut self, model: &M, view: View<M, S>) -> Vec<Command> {
        let dom = view(&model);
        let changeset = dom::diff(&self.dom, &dom);

        // Replace 'old' DOM with 'new' DOM
        self.dom = dom;

        commands(&self.dom, changeset)
    }
}
