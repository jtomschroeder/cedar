
use std::marker::PhantomData;

use dom;
use super::{View, Window, Label, Stack, Button};
use cacao::widget::Widget;

pub trait Viewable<S> {
    fn view(&self) -> View<S>;
}

impl<S, F> Viewable<S> for F
    where F: Fn() -> View<S>
{
    fn view(&self) -> View<S> {
        self()
    }
}

pub struct Program<S, M, U, V> {
    model: M,
    update: U,
    view: V,
    message: PhantomData<S>,
}

impl<S, M, U, V> Program<S, M, U, V> {
    pub fn new(model: M, update: U, view: V) -> Self {
        Program {
            model: model,
            update: update,
            view: view,
            message: PhantomData,
        }
    }
}

fn create(node: dom::Node) -> Box<Widget> {
    let mut widget: Box<Widget> = match node.kind {
        dom::Kind::Label => Box::new(Label::new()),
        dom::Kind::Button => Box::new(Button::new()), 
        dom::Kind::Stack => Box::new(Stack::new()),
    };

    for child in node.children.into_iter() {
        widget.add(create(child));
    }

    widget
}

impl<S, M, U, V> Program<S, M, U, V>
    where S: Send + 'static,
          M: Send + 'static,
          U: ::Update<M, S> + Send + 'static,
          V: Viewable<S>
{
    pub fn run(self) {
        let app = super::Application::new(); // TODO: enforce `app` created first

        let mut window = Window::new("cedar");

        {
            use dom;
            use dom::Kind;
            use dom::Attribute::*;
            use dom::Operation;

            // let u = node![Kind::Label |> Text("!".into())];

            let u = node![Kind::Stack => node![Kind::Button]
                                 , node![Kind::Label |> Text("!".into())]
                                 , node![Kind::Button]
                        ];

            // let u = node![Stack => node![Button]
            //                      , node![Label |> Text("!".into())]
            //                      , node![Button]
            //             ];

            println!("nodes: {:?}", u);

            // let changeset = dom::diff(vec![], vec![u]);
            // println!("changeset: {:#?}", changeset);

            // for (path, operation) in changeset.into_iter() {
            //     println!("{:?}", path);
            //     println!("{:?}", operation);

            //     // - traverse to `path`
            //     // - apply operation

            //     match operation {
            //         Operation::Create(node) => {
            //             match node.kind {
            //                 Kind::Label => window.add(Label::new()),
            //                 _ => {}
            //             }
            //         }
            //         _ => {}
            //     }
            // }

            // let label = Label::new();
            // let mut stack = Stack::new();
            // stack.add(Box::new(label));

            window.add(create(u));
        }



        // let mut view = self.view.view();

        // let mut model = self.model;
        // view.update(&model);

        // let mut update = self.update;
        app.run(move || loop {
                    // let message = view.stream().pop();
                    // model = update.update(&model, message);
                    // view.update(&model);
                })
    }
}
