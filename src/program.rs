
use std::str;
use std::collections::HashMap;
use std::process::{self, Stdio};
use std::io::BufReader;
use std::io::prelude::*;

use serde_json as json;

use yoga;
use dom;
use tree;

use tree::Vertex;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

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

    Move { id: Identifier, frame: Frame },

    Remove(Identifier), // ID
}

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Click { id: Identifier },
    Change { id: Identifier, value: String },

    Resize { width: f32, height: f32 },
}

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn convert<T: Clone>(
    dom: &dom::Object<T>,
    layout: &yoga::Node,
    set: dom::Changeset,
) -> Vec<Command> {
    let mut commands = vec![];

    fn expand<S>(
        _path: &tree::Path,
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
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
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

    let (mut width, mut height) = (500., 500.);

    let mut layout = yoga(&dom);
    layout.calculuate(width, height);

    // Create changeset: Create @ 'root'
    let patch = vec![(tree::Path::new(), tree::Operation::Create)];

    let commands = convert(&dom, &layout, patch);

    let mut stdin = output.stdin.unwrap();
    for event in commands.into_iter() {
        writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
    }

    /// Receive messages from 'renderer' process (via stdout)

    let stdout = BufReader::new(output.stdout.unwrap());
    for line in stdout.lines().filter_map(|line| line.ok()) {
        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        let event = match json::from_str(&line) {
            Ok(event) => event,
            Err(err) => {
                println!("Failed to parse event: '{}' :: {:?}", line, err);
                continue;
            }
        };

        enum Action<S> {
            Update(S),
            Layout(f32, f32),
        }

        let action = match event {
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

            Event::Resize { width, height } => Some(Action::Layout(width, height)),
        };

        let action = match action {
            Some(a) => a,
            _ => continue,
        };

        // TODO: some commands from renderer (e.g. window resize) will not generate 'message' to `update`
        //   but will (potentially) require re-yoga
        // - no `update` means call to `view` i.e. no new `dom`

        match action {
            Action::Update(message) => {
                model = update(model, message);
            }

            Action::Layout(w, h) => {
                width = w;
                height = h;
            }
        }

        let old = dom;
        dom = view(&model);

        let changeset = dom::diff(&old, &dom);

        let old_layout = layout;
        layout = yoga(&dom);
        layout.calculuate(width, height);

        let mut commands = convert(&dom, &layout, changeset);

        old_layout.merge(
            &layout,
            |path, old, new| if old.left() != new.left() || old.top() != new.top() ||
                old.width() != new.width() ||
                old.height() != new.height()
            {
                let id = path.to_string();
                let frame = (new.left(), new.top(), new.width(), new.height());
                commands.push(Command::Move { id, frame })
            },
        );

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
