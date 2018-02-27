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

fn words(line: &str) -> cedar::dom::List<Widget> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|w| {
            // TODO: hypertext! { <div>{w}</div> }
            hypertext! { <div>{"word"}</div> }
        })
        .collect()
}

// Use global as workaround for https://github.com/rust-lang/rust/issues/46489
static mut MODEL: Option<Model> = None;

fn view(model: &Model) -> Widget {
    unsafe {
        MODEL = Some(model.clone());

        trace_macros!(true);
        hypertext! {
            <div>
                <input placeholder={"Words!"} input={Message::NewContent}></input>
                <div>{words(MODEL.as_ref().unwrap())}</div>
                // TODO: <div>{words(model)}</div>
            </div>
        }
    }
}

fn main() {
    cedar::program("".into(), update, view)
}
