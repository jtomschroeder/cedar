#![feature(trace_macros)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate cedar;

use cedar::hypertext;

type Model = String;

#[derive(PartialEq)]
enum Message {
    NewContent(String),
}

fn update(_: Model, message: &Message) -> Model {
    match message {
        &Message::NewContent(ref content) => content.clone(),
    }
}

type Widget = cedar::dom::Object<Message>;

fn words(line: &str) -> cedar::dom::List<Widget> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|w| (hypertext! { |w| <div>{w}</div> })(w))
        .collect()
}

fn view(model: &Model) -> Widget {
    // trace_macros!(true);
    (hypertext! { |model|
        <div>
            <input placeholder={"Words!"} input={Message::NewContent}></input>
            <div>{words(model)}</div>
        </div>
    })(model)
}

fn main() {
    cedar::program("".into(), update, view)
}
