
use std::fmt::Debug;
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

    use std::process::Command;
    let output = Command::new("./cocoa/target/release/cocoa")
        .output()
        .expect("failed to execute process");

    println!("output: {:?}", output.stdout);
}
