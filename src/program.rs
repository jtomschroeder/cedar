
use std::str;
use std::fmt::Debug;
use std::process::{Command, Stdio};

use std::io::BufReader;
use std::io::prelude::*;

use serde_json as json;

use dom;
use tree;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

// TODO: investigate using Godel numbering of lists to encode 'path' of widget as usize ID
// - might be easier than allocating vectors for each child

type Identifier = String;

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Create(Identifier, String), // TODO: ID, Attributes (e.g. Text), Location (i.e. 'frame')
    Update(Identifier, String, String), // ID -> Attribute
    Remove(Identifier), // ID
}

/// Convert 'changeset' to list of events to send to UI 'rendering' process
fn convert<T: Clone>(dom: &dom::Object<T>, set: dom::Changeset) -> Vec<Event> {
    let mut events = vec![];

    fn expand<S>(path: tree::Path, node: &dom::Object<S>, events: &mut Vec<Event>) {
        // TODO: use breadth-first traversal here (using queue) - use path!

        let (ref kind, _) = node.value;

        // TODO: this code is duplicated below (DRY)
        // Create string representation of path (e.g. 0.0.1.3)
        let id = if path.is_empty() {
            String::new()
        } else if path.len() == 1 {
            path[0].to_string()
        } else {
            (&path[1..]).iter().fold(path[0].to_string(), |id, n| {
                id + &format!(".{}", n)
            })
        };

        match kind {
            &dom::Kind::Label => events.push(Event::Create(id, "Label".into())),
            &dom::Kind::Button => events.push(Event::Create(id, "Button".into())),
            &dom::Kind::Field => events.push(Event::Create(id, "Field".into())),
            _ => {}
        }

        for (n, child) in node.children.iter().enumerate() {
            let mut path = path.clone();
            path.push(n);

            expand(path, child, events);
        }
    }


    for (path, op) in set.into_iter() {
        let dom = dom.clone();
        let nodes = vec![dom];
        let node = find(&path, &nodes).unwrap();

        match op {
            tree::Operation::Create => expand(path, node, &mut events),
            tree::Operation::Update => {

                let (_, ref attrs) = node.value;

                // Create string representation of path (e.g. 0.0.1.3)
                let id = if path.is_empty() {
                    String::new()
                } else if path.len() == 1 {
                    path[0].to_string()
                } else {
                    (&path[1..]).iter().fold(path[0].to_string(), |id, n| {
                        id + &format!(".{}", n)
                    })
                };

                for attr in attrs {
                    match attr {
                        &dom::Attribute::Text(ref txt) => {
                            events.push(Event::Update(id.clone(), "Text".into(), txt.clone()))
                        }
                        _ => {}
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    events
}

fn find<'s, S>(path: &[usize], nodes: &'s [dom::Object<S>]) -> Option<&'s dom::Object<S>> {
    if path.is_empty() {
        None
    } else if path.len() == 1 {
        Some(&nodes[path[0]])
    } else {
        for node in nodes {
            let obj = find(&path[1..], &node.children);
            if obj.is_some() {
                return obj;
            }
        }
        None
    }
}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    let mut dom = view(&model);

    // let tree = tree::Tree { children: vec![dom] };

    // println!("model: {:?}", model);
    // println!("view: {:?}", dom);

    // TODO: use `spawn` and listen to stdin/stdout
    // - implement 'quit' event (or just exit when process terminates)

    // TODO: remove hard-coded path to UI subprocess exe
    // - `fork` is another option - only *nix compatible, though.

    let output = Command::new("./cocoa/target/release/cocoa")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    // Create changeset: Create @ 'root'
    let patch = vec![(vec![0], tree::Operation::Create)];

    let events = convert(&dom, patch);

    let mut stdin = output.stdin.unwrap();
    for event in events.into_iter() {
        writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
    }

    let stdout = BufReader::new(output.stdout.unwrap());
    for line in stdout.lines() {
        // println!("from renderer: {:?}", line);

        let line = line.unwrap();

        let mut split = line.split(".");
        let command = split.next().unwrap();

        let path: Vec<usize> = split.map(|s| s.parse().unwrap()).collect();

        // println!("from renderer: {:?} :: {:?}", command, path);

        let message = match command {
            "click" => {
                // TODO: move 'find' logic into tree/dom module

                let nodes = &[dom.clone()];
                let node = find(&path, nodes);
                // println!("{:?}", node);

                let node = match node {
                    Some(node) => node,
                    _ => continue,
                };

                // TODO: refactor this!
                // - possibly make attributes members of struct instead of vector?

                let mut message = None;
                let (_, ref attrs) = node.value;
                for attr in attrs {
                    match attr {
                        &dom::Attribute::Click(ref e) => {
                            message = Some(e.clone());
                            break;
                        }
                        _ => {}
                    }
                }

                match message {
                    Some(message) => message,
                    _ => continue,
                }
            }

            _ => continue,
        };


        // println!("message: {:?}", message);

        model = update(model, message);

        let old = dom;
        dom = view(&model);

        // println!("node: {:?}", new);

        let changeset = dom::diff(&old, &dom);
        // println!("changeset: {:?}", changeset);

        let events = convert(&dom, changeset);

        // let mut stdin = output.stdin.unwrap();
        for event in events.into_iter() {
            // println!("event: {:?}", event);
            writeln!(stdin, "{}", json::to_string(&event).unwrap()).unwrap();
        }
    }
}
