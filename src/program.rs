
use std::str;
use std::fmt::Debug;
use std::process::{Command, Stdio};

use std::io::BufReader;
use std::io::prelude::*;

use dom;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    let view = view(&model);

    println!("model: {:?}", model);
    println!("view: {:?}", view);

    // TODO: use `spawn` and listen to stdin/stdout
    // - implement 'quit' event (or just exit when process terminates)

    let output = Command::new("./cocoa/target/release/cocoa")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    println!("WAITING");

    // let stdout = BufReader::new(output.stdout.unwrap());
    // for line in stdout.lines() {
    //     println!("{:?}", line);
    // }

    let mut buffer = vec![0; 1024];
    let mut stdout = output.stdout.unwrap();
    loop {
        match stdout.read(&mut buffer) {
            Ok(0) | Err(_) => break,
            Ok(len) => {
                println!("{:?}", str::from_utf8(&buffer[..len]));
            }
        }
    }
}
