use std::str;
use std::collections::HashMap;

use dom;
use tree::{self, Vertex};
use renderer::{Command, Event, Update};
use program::View;

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn commands<T>(
    old: Option<&dom::Object<T>>,
    dom: &dom::Object<T>,
    set: dom::Changeset,
) -> Vec<Command> {
    let mut commands = vec![];

    fn expand<S>(root: &tree::Path, node: &dom::Object<S>, commands: &mut Vec<Command>) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.traverse(root, |path, node| {
            let id = path.to_string();

            let kind = node.widget.element();
            let value = node.widget.value.clone();

            let attributes = node.attributes.iter().map(|attr| attr.raw()).collect();

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
                // TODO: are we missing an update to 'Text' attributes?

                let node = node();
                if node.widget.is_text() {
                    let value = node.widget.value.clone().unwrap();
                    commands.push(Command::Update {
                        id: id(),
                        value: Update::Text(value),
                    })
                } else {
                    let mut attrs: HashMap<_, _> =
                        node.attributes.iter().map(|attr| attr.raw()).collect();

                    // Clear out any attributes that are no longer used.
                    if let Some(old) = old {
                        for (key, _) in old.attributes.iter().map(|attr| attr.raw()) {
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

pub struct Phantom<S> {
    dom: dom::Object<S>,
}

impl<S> Phantom<S>
where
    S: 'static + Send + PartialEq + Clone,
{
    pub fn initialize<M>(model: &M, view: View<M, S>) -> (Self, Vec<Command>) {
        let dom = view(&model);

        // Create changeset: Create @ 'root'
        let patch = vec![(tree::Path::new(), tree::Operation::Create)];

        let commands = commands(None, &dom, patch);

        (Phantom { dom }, commands)
    }

    /// Find the message associated with an event (by looking up node in DOM)
    pub fn translate(&self, event: Event) -> Option<S> {
        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        let path = |id: &str| {
            let path = id.split(".").filter_map(|s| s.parse().ok()).collect();
            tree::Path::from_vec(path)
        };

        let ref dom = self.dom;
        match event {
            Event::Click { id } => {
                let path = path(&id);
                dom.find(&path).and_then(|node| node.widget.click.clone())
            }

            Event::Input { id, value } => {
                let path = path(&id);
                dom.find(&path)
                    .and_then(|node| node.widget.input.as_ref().map(|i| i(value)))
            }

            Event::Keydown { id, code } => {
                let path = path(&id);
                dom.find(&path)
                    .and_then(|node| node.widget.keydown.as_ref().and_then(|k| k(code)))
            }
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
