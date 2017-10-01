
use std::str;
use std::fmt::Debug;
use std::process::{Command, Stdio, ChildStdin};
use std::collections::VecDeque;

use std::io::BufReader;
use std::io::prelude::*;

use serde_json as json;

use dom;
use tree;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Create(String), // TODO: ID, Attributes (e.g. Text), Location (i.e. 'frame')
    Update, // ID -> Attribute
    Remove, // ID
}

/// Convert 'changeset' to list of events to send to UI 'rendering' process
fn convert<T>(set: dom::Changeset<T>) -> Vec<Event> {
    let mut events = vec![];

    fn expand<S>(location: tree::Location, node: dom::Object<S>, events: &mut Vec<Event>) {
        // TODO: use breadth-first traversal here (using queue) - use location!

        let (kind, _) = node.value;

        match kind {
            dom::Kind::Label => events.push(Event::Create("Label".into())),
            dom::Kind::Button => events.push(Event::Create("Button".into())),
            dom::Kind::Field => events.push(Event::Create("Field".into())),
            _ => {}
        }

        for child in node.children.into_iter() {
            expand(location.clone(), child, events);
        }
    }

    for (location, op) in set.into_iter() {
        match op {
            tree::Operation::Create(node) => expand(location, node, &mut events),
            _ => unimplemented!(),
        }
    }

    events
}

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    let dom = view(&model);

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

    let patch = dom::build(dom);
    let events = convert(patch);

    let mut stdin = output.stdin.unwrap();
    for event in events.into_iter() {
        writeln!(stdin, "{}", json::to_string(&event).unwrap());
    }

    let stdout = BufReader::new(output.stdout.unwrap());
    for line in stdout.lines() {
        println!("{:?}", line);
    }
}
