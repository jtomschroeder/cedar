
use std::str;
use std::fmt::Debug;
use std::process::{self, Stdio};
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};

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
type Frame = (f32, f32, f32, f32); // (x, y, w, h)

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Create {
        id: Identifier,
        kind: String,
        frame: Frame,
        attributes: HashMap<String, String>,
    },

    Update(Identifier, String, String), // ID -> Attribute

    Remove(Identifier), // ID
}

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Click { id: Identifier },
    Change { id: Identifier, value: String },
}

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn convert<T: Clone>(
    dom: &dom::Object<T>,
    layout: &yoga::Node,
    set: dom::Changeset,
) -> Vec<Command> {
    let mut commands = vec![];

    fn expand<S>(
        path: &tree::Path,
        node: &dom::Object<S>,
        layout: &yoga::Node,
        commands: &mut Vec<Command>,
    ) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.merge(layout, |path, node, layout| {
            let id = path.to_string();

            let frame = (layout.left(), layout.top(), layout.width(), layout.height());

            let mut attributes = HashMap::new();

            let kind = match node.widget {
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

                _ => None,
            };

            if let Some(kind) = kind {
                commands.push(Command::Create {
                    id,
                    kind,
                    frame,
                    attributes,
                })
            }
        });
    }

    for (path, op) in set.into_iter() {
        let node = dom.find(&path).expect("path in nodes");

        match op {
            tree::Operation::Create => expand(&path, node, layout, &mut commands),
            tree::Operation::Update => {
                let id = path.to_string();
                match node.widget {
                    dom::Widget::Label(ref label) => {
                        commands.push(Command::Update(id, "Text".into(), label.text.clone()))
                    }

                    _ => unimplemented!(),
                }
            }

            _ => unimplemented!(),
        }
    }

    commands
}

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
    let output = process::Command::new("./cocoa/target/release/cocoa")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let mut dom = view(&model);

    let layout = yoga(&dom);
    layout.calculuate();

    // Create changeset: Create @ 'root'
    let patch = vec![(tree::Path::new(), tree::Operation::Create)];

    let commands = convert(&dom, &layout, patch);

    let mut stdin = output.stdin.unwrap();
    for event in commands.into_iter() {
        writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
    }

    /// Receive messages from 'renderer' process (via stdout)

    let stdout = BufReader::new(output.stdout.unwrap());
    for event in stdout.lines().filter_map(|line| {
        // TODO: refactor this?
        line.ok().and_then(|line| json::from_str(&line).ok())
    })
    {
        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        let message = match event {
            Event::Click { id } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Button(ref button) => button.click.clone(),
                    _ => None,
                })
            }

            Event::Change { id, value } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Field(ref field) => field.change.map(|c| c(value)),
                    _ => None,
                })
            }
        };

        let message = match message {
            Some(m) => m,
            _ => continue,
        };

        // TODO: some commands from renderer (e.g. window resize) will not generate 'message' to `update`
        //   but will (potentially) require re-yoga
        // - no `update` means call to `view` i.e. no new `dom`

        model = update(model, message);

        let old = dom;
        dom = view(&model);

        let changeset = dom::diff(&old, &dom);

        // TODO: generate layout for `dom`
        // TODO: pass `layout` to `convert` to be associated with commands (to renderer)

        let layout = yoga(&dom);
        layout.calculuate();

        let commands = convert(&dom, &layout, changeset);

        for event in commands.into_iter() {
            writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
        }
    }
}

fn yoga<T>(node: &dom::Object<T>) -> yoga::Node {
    let mut layout = yoga::Node::new();

    // TODO: 'Flow' => Row
    // TODO: likely need to treat 'root' node differently

    match node.widget {
        dom::Widget::Stack => layout.set_direction(), // Column

        dom::Widget::Button(_) |
        dom::Widget::Label(_) |
        dom::Widget::Field(_) => {
            layout.set_margin(20.);

            layout.set_min_height(24.);
            layout.set_max_height(24.);
        }
    }

    // Traverse children, building nodes 'bottom-up'
    for (n, node) in node.children().iter().map(yoga).enumerate() {
        layout.insert(node, n as u32);
    }

    layout
}

impl tree::Vertex for yoga::Node {
    fn children(&self) -> &[Self] {
        self.children()
    }
}
