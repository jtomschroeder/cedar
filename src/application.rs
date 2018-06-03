use json;

use std::fs::File;
use std::io::prelude::*;

use sass;
use web_view;

use dom;
use renderer;
use shadow::Shadow;

pub type Update<M, S> = fn(M, &S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub struct Application<M, S> {
    model: M,
    update: Update<M, S>,
    view: View<M, S>,

    style: Option<String>,
}

impl<M, S> Application<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    pub fn new(model: M, update: Update<M, S>, view: View<M, S>) -> Self {
        Application {
            model,
            update,
            view,

            style: None,
        }
    }

    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.into());
        self
    }

    pub fn run(self) {
        Program::run(self.model, self.update, self.view, self.style)
    }
}

pub fn app<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
    where
        S: Send + PartialEq + 'static,
        M: Send + 'static,
{
    Application::new(model, update, view).run()
}

struct Program<M, S> {
    model: Option<M>,
    update: Update<M, S>,
    view: View<M, S>,
    shadow: Shadow<S>,
}

const HTML: &str = include_str!("../lib/web-view/index.html");
const CSS: &str = include_str!("../lib/web-view/style.scss");

impl<M, S> Program<M, S>
where
    S: Send + PartialEq + 'static,
    M: Send + 'static,
{
    fn new(model: M, update: Update<M, S>, view: View<M, S>) -> (Self, Vec<renderer::Command>) {
        let (shadow, commands) = Shadow::initialize(&model, view);

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

        commands
    }

    fn run(model: M, update: Update<M, S>, view: View<M, S>, style: Option<String>) {
        let (mut program, mut commands) = Program::new(model, update, view);

        let html = match style {
            Some(style) => HTML.replace("/* styles */", &style),
            _ => {
                let css = sass::compile_string(CSS, sass::Options::default()).unwrap();
                HTML.replace("/* styles */", &css)
            }
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
}
