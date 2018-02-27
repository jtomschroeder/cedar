#![feature(proc_macro)]
#![feature(trace_macros)]

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

fn words(line: &str) -> Vec<Widget> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|w| {
            hypertext! { <div>{"w"}</div> }
        })
        .collect()
}

// Use global as workaround for https://github.com/rust-lang/rust/issues/46489
static mut MODEL: Model = "".into();

fn view(model: &Model) -> Widget {
    unsafe {
        MODEL = model.clone();

        trace_macros!(true);
        hypertext! {
            <div>
                <input placeholder={"Words!"} input={Message::NewContent}></input>
                <div>{words("hello world")}</div>
            </div>
        }
    }
}

fn main() {
    cedar::program("".into(), update, view)
}
