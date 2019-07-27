use std::collections::HashMap;
use std::str;

use crate::boo::Boo;

use crate::application::View;
use crate::dom;
use crate::dom::Attribute;
use crate::renderer::{Command, Event, Update};
use crate::tree::{self, Vertex};

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn commands<T>(
    old: Option<&dom::Object<T>>,
    dom: &dom::Object<T>,
    set: dom::Changeset,
) -> Vec<Command> {
    fn expand<S>(root: &tree::Path, node: &dom::Object<S>, commands: &mut Vec<Command>) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.traverse(root, |path, node| {
            let id = path.to_string();

            let kind = node.element();
            let value = node.value.clone();

            let attributes = node.attributes.iter().flat_map(|attr| attr.raw()).collect();

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

    let mut commands = vec![];

    for (path, op) in set.into_iter() {
        let node = || dom.find(&path).expect("path in nodes");
        let id = || path.to_string();

        match op {
            tree::Operation::Create => expand(&path, node(), &mut commands),
            tree::Operation::Update => {
                // TODO: are we missing an update to 'Text' attributes?

                let node = node();
                if node.is_text() {
                    let value = node.value.clone().unwrap();
                    commands.push(Command::Update {
                        id: id(),
                        value: Update::Text(value),
                    })
                } else {
                    let mut attrs: HashMap<_, _> =
                        node.attributes.iter().flat_map(|attr| attr.raw()).collect();

                    // Clear out any attributes that are no longer used.
                    if let Some(old) = old {
                        for (key, _) in old.attributes.iter().flat_map(|attr| attr.raw()) {
                            if !attrs.contains_key(&key) {
                                attrs.insert(key, "".into());
                            }
                        }
                    }

                    commands.push(Command::Update {
                        id: id(),
                        value: Update::Attributes(attrs),
                    })
                }
            }

            tree::Operation::Delete => commands.push(Command::Remove { id: id() }),

            tree::Operation::Replace => panic!("`Replace` not yet implemented!"),
        }
    }

    commands
}

pub struct Shadow<S> {
    dom: dom::Object<S>,
}

impl<S> Shadow<S>
where
    S: Send + PartialEq + 'static,
{
    pub fn initialize<M>(model: &M, view: View<M, S>) -> (Self, Vec<Command>) {
        let dom = view(&model);

        // Create changeset: Create @ 'root'
        let patch = vec![(tree::Path::new(), tree::Operation::Create)];

        let commands = commands(None, &dom, patch);

        (Shadow { dom }, commands)
    }

    fn find(&self, id: &str) -> Option<&dom::Object<S>> {
        let path = id.split(".").filter_map(|s| s.parse().ok()).collect();
        let path = tree::Path::from_vec(path);

        self.dom.find(&path)
    }

    /// Find the message associated with an event (by looking up node in DOM)
    pub fn translate(&self, event: Event) -> Option<Boo<S>> {
        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        match event {
            Event::Click { id } => self.find(&id).and_then(|node| {
                for attr in node.attributes.iter() {
                    match attr {
                        Attribute::Click(value) => return Some(Boo::Borrowed(value)),
                        _ => {}
                    }
                }

                None
            }),

            Event::Input { id, value } => self.find(&id).and_then(|node| {
                for attr in node.attributes.iter() {
                    match attr {
                        Attribute::Input(input) => return Some(Boo::Owned(input(value))),
                        _ => {}
                    }
                }

                None
            }),

            Event::Keydown { id, code } => self.find(&id).and_then(|node| {
                for attr in node.attributes.iter() {
                    match attr {
                        Attribute::Keydown(keydown) => return keydown(code).map(Boo::Owned),
                        _ => {}
                    }
                }

                None
            }),
        }
    }

    pub fn update<M>(&mut self, model: &M, view: View<M, S>) -> Vec<Command> {
        let dom = view(&model);
        let changeset = dom::diff(&self.dom, &dom);

        let cmds = commands(Some(&self.dom), &dom, changeset);

        // Replace 'old' DOM with 'new' DOM
        self.dom = dom;

        cmds
    }
}
