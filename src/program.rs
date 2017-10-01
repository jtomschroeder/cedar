
use std::str;
use std::fmt::Debug;
use std::process::{Command, Stdio, ChildStdin};

use std::io::BufReader;
use std::io::prelude::*;

use serde_json as json;

use dom;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Create(String), // TODO: Attributes (e.g. Text), Location (i.e. 'frame')
    Update,
    Remove,
}

pub fn create<S: Clone + Debug + 'static>(stdin: &mut ChildStdin, node: dom::Object<S>) {
    let (kind, attributes) = node.value;

    // println!("create: {:?} with {:?}", kind, attributes);

    let event = match kind {
        dom::Kind::Label => Some(Event::Create("Label".into())),
        dom::Kind::Button => Some(Event::Create("Button".into())),
        dom::Kind::Field => Some(Event::Create("TextField".into())),
        _ => None,
    };

    if let Some(event) = event {
        writeln!(stdin, "{}", json::to_string(&event).unwrap());
    }

    for child in node.children.into_iter() {
        create(stdin, child);
    }
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

    // println!("WAITING");

    let mut stdin = output.stdin.unwrap();
    create(&mut stdin, dom);

    // writeln!(stdin, "ACTION");

    let stdout = BufReader::new(output.stdout.unwrap());
    for line in stdout.lines() {
        println!("{:?}", line);
    }

    // let mut buffer = vec![0; 1024];
    // let mut stdout = output.stdout.unwrap();
    // loop {
    //     match stdout.read(&mut buffer) {
    //         Ok(0) | Err(_) => break,
    //         Ok(len) => {
    //             println!("{:?}", str::from_utf8(&buffer[..len]));
    //         }
    //     }
    // }
}
