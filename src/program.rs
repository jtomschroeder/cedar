use json;

use std;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

use web_view;

use browser;
use dom;
use processor;
use renderer;
use shadow::Shadow;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

struct Program<M, S> {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    shadow: Shadow<S>,
}

impl<M, S> Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn new(model: M, update: Update<M, S>, view: View<M, S>) -> (Self, Vec<renderer::Command>) {
        let (shadow, commands) = Shadow::initialize(&model, view);

        // Self::send(commands);

        (
            Program {
                model: Some(model),
                update,
                view,
                shadow,
            },
            commands,
        )
    }

    //    fn send(commands: Vec<renderer::Command>) {
    //        for cmd in commands.into_iter() {
    //            let cmd = json::to_string(&cmd).unwrap();
    //            browser::command(&cmd);
    //        }
    //    }

    fn process(&mut self, event: &str) -> Vec<renderer::Command> {
        let event: renderer::Event = json::from_str(event).unwrap();

        // TODO: get new subscriptions
        // - Do a 'difference' on the old and new
        // - Enable new ones and disable old ones

        let model = {
            // translate events from backend renderer to actions
            let message = match self.shadow.translate(event) {
                Some(m) => m,
                _ => return vec![], // TODO: Option<>?
            };

            let model = self.model.take().unwrap();
            (self.update)(model, &message)
        };

        let commands = {
            let commands = self.shadow.update(&model, self.view);
            self.model = Some(model);
            commands
        };

        // Self::send(commands);

        commands
    }
}

//impl<M, S> processor::Processor for Program<M, S>
//where
//    S: Send + PartialEq + 'static,
//    M: Send + 'static,
//{
//    fn process(&mut self, event: String) {
//        Program::process(self, event)
//    }
//}

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    let (mut program, mut commands) = Program::new(model, update, view);
    // processor::initialize(program);

    println!("{:?}", commands);

    let html = {
        let mut file = File::open("lib/web-view/index.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let title = "cedar app";

    let size = (800, 600);
    let resizable = true;
    let debug = true;

    web_view::run(
        title,
        web_view::Content::Html(html),

        Some(size),
        resizable,
        debug,

        move |webview| {
            webview.dispatch(move |webview, _| {
                webview.eval("setup()");

                for cmd in commands.drain(..) {
                    let cmd = json::to_string(&cmd).unwrap();
                    webview.eval(&format!("window.cedar.command('{}')", cmd));
                }
            });
        },

        move |webview, message, _| {
            println!("message: {}", message);

            let mut commands = program.process(message);

            for cmd in commands.drain(..) {
                let cmd = json::to_string(&cmd).unwrap();
                webview.eval(&format!("window.cedar.command('{}')", cmd));
            }
        },

        (),
    );
}
