
use std::str;
use std::fmt::Debug;
use std::process::{Command, Stdio};
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;

use serde_json as json;

use yoga;
use dom;
use tree;

use tree::Vertex;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

// TODO: investigate using Godel numbering of lists to encode 'path' of widget as usize ID
// - might be easier than allocating vectors for each child

type Identifier = String;

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    // TODO: ID, Attributes (e.g. Text), Location (i.e. 'frame')
    // TODO: `text` should really be generic list of 'attributes'
    Create {
        id: Identifier,
        kind: String,
        text: String,
    },

    Update(Identifier, String, String), // ID -> Attribute

    Remove(Identifier), // ID
}

/// Convert 'changeset' to list of events to send to UI 'rendering' process
fn convert<T: PartialEq + Clone>(
    dom: &dom::Object<T>,
    layout: &yoga::Node,
    set: dom::Changeset,
) -> Vec<Event> {
    let mut events = vec![];

    fn expand<S: PartialEq>(
        path: &tree::Path,
        node: &dom::Object<S>,
        layout: &yoga::Node,
        events: &mut Vec<Event>,
    ) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.merge(layout, |path, node, layout| {
            let id = path.to_string();

            println!("layout: {:?} :: {:?}", path, layout);

            match node.widget {
                dom::Widget::Label(ref label) => {
                    events.push(Event::Create {
                        id,
                        kind: "Label".into(),
                        text: label.text.clone(),
                    })
                }

                dom::Widget::Button(ref button) => {
                    events.push(Event::Create {
                        id,
                        kind: "Button".into(),
                        text: button.text.clone(),
                    })
                }

                dom::Widget::Field(_) => {
                    events.push(Event::Create {
                        id,
                        kind: "Field".into(),
                        text: "".into(),
                    })
                }

                _ => {}
            }
        });
    }

    for (path, op) in set.into_iter() {
        let node = dom.find(&path).expect("path in nodes");

        match op {
            tree::Operation::Create => expand(&path, node, layout, &mut events),
            tree::Operation::Update => {
                let id = path.to_string();
                match node.widget {
                    dom::Widget::Label(ref label) => {
                        events.push(Event::Update(id, "Text".into(), label.text.clone()))
                    }

                    _ => unimplemented!(),
                }
            }

            _ => unimplemented!(),
        }
    }

    events
}

// fn render() {}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    // TODO: use `spawn` and listen to stdin/stdout
    // - implement 'quit' event (or just exit when process terminates)

    // TODO: remove hard-coded path to UI subprocess exe
    // - `fork` is another option - only *nix compatible, though.

    // start 'renderer' subprocess
    let output = Command::new("./cocoa/target/release/cocoa")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let mut dom = view(&model);

    let layout = yoga(&dom);
    layout.calculuate();

    // Create changeset: Create @ 'root'
    let patch = vec![(tree::Path::new(), tree::Operation::Create)];

    let events = convert(&dom, &layout, patch);

    let mut stdin = output.stdin.unwrap();
    for event in events.into_iter() {
        writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
    }

    /// Receive messages from 'renderer' process (via stdout)

    let stdout = BufReader::new(output.stdout.unwrap());
    for line in stdout.lines() {
        // TODO: define/implement this API using JSON

        let line = line.unwrap();
        let mut split = line.split(".");
        let command = split.next().unwrap();
        let path = tree::Path::from_vec(split.map(|s| s.parse().unwrap()).collect());

        let message = match command {
            "click" => {
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Button(ref button) => button.click.clone(),
                    _ => None,
                })
            }

            _ => None,
        };

        let message = match message {
            Some(m) => m,
            _ => continue,
        };

        // TODO: some events from renderer (e.g. window resize) will not generate 'message' to `update`
        //   but will (potentially) require re-yoga
        // - no `update` means call to `view` i.e. no new `dom`

        model = update(model, message);

        let old = dom;
        dom = view(&model);

        let changeset = dom::diff(&old, &dom);

        // TODO: generate layout for `dom`
        // TODO: pass `layout` to `convert` to be associated with events (to renderer)

        let layout = yoga(&dom);
        layout.calculuate();

        let events = convert(&dom, &layout, changeset);

        for event in events.into_iter() {
            writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
        }
    }
}

fn yoga<V: Vertex>(tree: &V) -> yoga::Node {
    let mut root = yoga::Node::new();

    // Traverse children, building nodes 'bottom-up'

    for (n, node) in tree.children().iter().map(yoga).enumerate() {
        root.insert(node, n as u32);
    }

    root
}

impl tree::Vertex for yoga::Node {
    fn children(&self) -> &[Self] {
        self.children()
    }

    fn compare(&self, other: &Self) -> Option<tree::Difference> {
        None
    }
}
