#![feature(proc_macro)]
#![feature(trace_macros)]

extern crate cedar;

use cedar::hypertext;

type Model = ();

#[derive(PartialEq)]
enum Message {}

fn update(model: Model, _message: &Message) -> Model {
    model
}

type Widget = cedar::dom::Object<Message>;

fn view(model: &Model) -> Widget {
    (hypertext! { || <div>Hello</div> })()
}

fn subscriptions(_: &Model) {}

fn main() {
    cedar::programv(((), update, view, subscriptions))
}
