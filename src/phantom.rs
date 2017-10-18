
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

            let kind = match node.widget {
                dom::Widget::Stack => {
                    // TODO: unimplemented!()
                    None
                }

                dom::Widget::Label(ref label) => {
                    attributes.insert("Text".into(), label.text.clone());
                    Some("Label".into())
                }

                dom::Widget::Button(ref button) => {
                    attributes.insert("Text".into(), button.text.clone());
                    Some("Button".into())
                }

                dom::Widget::Field(ref field) => {
                    if let Some(ref placeholder) = field.placeholder {
                        attributes.insert("Placeholder".into(), placeholder.clone());
                    }
                    Some("Field".into())
                }
            };

            if let Some(kind) = kind {
                commands.push(Command::Create {
                    id,
                    kind,
                    attributes,
                })
            }
        });
    }

    for (path, op) in set.into_iter() {
        let node = || dom.find(&path).expect("path in nodes");
        let id = || path.to_string();

        match op {
            tree::Operation::Create => expand(&path, node(), &mut commands),
            tree::Operation::Update => {
                let node = node();
                match node.widget {
                    dom::Widget::Label(ref label) => {
                        commands.push(Command::Update(id(), "Text".into(), label.text.clone()))
                    }

                    _ => panic!("`Update` not yet implemented for widget!"),
                }
            }

            tree::Operation::Delete => commands.push(Command::Remove(id())),

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

        let ref dom = self.dom;
        match event {
            Event::Click { id } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Button(ref button) => button.click.clone().map(Action::Update),
                    _ => None,
                })
            }

            Event::Change { id, value } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Field(ref field) => {
                        field.change.map(|c| c(value)).map(Action::Update)
                    }
                    _ => None,
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
