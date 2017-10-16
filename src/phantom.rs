
use std::str;
use std::collections::HashMap;

use yoga;
use dom;
use tree;

use tree::Vertex;

use renderer::{Command, Event};
use program::{View, Action};

/// Convert 'changeset' to list of commands to send to UI 'rendering' process
fn convert<T: Clone>(
    dom: &dom::Object<T>,
    layout: &yoga::Node,
    set: dom::Changeset,
) -> Vec<Command> {
    let mut commands = vec![];

    fn expand<S>(
        _path: &tree::Path,
        node: &dom::Object<S>,
        layout: &yoga::Node,
        commands: &mut Vec<Command>,
    ) {
        // TODO: handle create path issue (vertex traversal assumes from root)

        node.merge(layout, |path, node, layout| {
            let id = path.to_string();

            let frame = (layout.left(), layout.top(), layout.width(), layout.height());

            let mut attributes = HashMap::new();

            let kind = match node.widget {
                dom::Widget::Label(ref label) => {
                    attributes.insert("Text".into(), label.text.clone());
                    Some("Label".into())
                }

                dom::Widget::Button(ref button) => {
                    attributes.insert("Text".into(), button.text.clone());
                    Some("Button".into())
                }

                dom::Widget::Field(ref field) => {
                    if let Some(ref placeholder) = field.placeholder {
                        attributes.insert("Placeholder".into(), placeholder.clone());
                    }
                    Some("Field".into())
                }

                _ => None,
            };

            if let Some(kind) = kind {
                commands.push(Command::Create {
                    id,
                    kind,
                    frame,
                    attributes,
                })
            }
        });
    }

    for (path, op) in set.into_iter() {
        let node = dom.find(&path).expect("path in nodes");

        match op {
            tree::Operation::Create => expand(&path, node, layout, &mut commands),
            tree::Operation::Update => {
                let id = path.to_string();
                match node.widget {
                    dom::Widget::Label(ref label) => {
                        commands.push(Command::Update(id, "Text".into(), label.text.clone()))
                    }

                    _ => unimplemented!(),
                }
            }

            _ => unimplemented!(),
        }
    }

    commands
}

pub struct Phantom<S> {
    dom: dom::Object<S>,
    layout: yoga::Node,
}

impl<S> Phantom<S>
where
    S: Clone + Send + 'static + PartialEq,
{
    pub fn initialize<M>(
        model: &M,
        view: View<M, S>,
        width: f32,
        height: f32,
    ) -> (Self, Vec<Command>) {
        let dom = view(&model);

        let layout = yoga(&dom);
        layout.calculuate(width, height);

        // Create changeset: Create @ 'root'
        let patch = vec![(tree::Path::new(), tree::Operation::Create)];

        let commands = convert(&dom, &layout, patch);

        (Phantom { dom, layout }, commands)
    }

    pub fn translate(&self, event: Event) -> Option<Action<S>> {

        // TODO: serialize ID as Path object to avoid parsing!
        // - in both Command and Event

        let ref dom = self.dom;
        match event {
            Event::Click { id } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Button(ref button) => button.click.clone().map(Action::Update),
                    _ => None,
                })
            }

            Event::Change { id, value } => {
                let path =
                    tree::Path::from_vec(id.split(".").filter_map(|s| s.parse().ok()).collect());
                dom.find(&path).and_then(|node| match node.widget {
                    dom::Widget::Field(ref field) => {
                        field.change.map(|c| c(value)).map(Action::Update)
                    }
                    _ => None,
                })
            }

            Event::Resize { width, height } => Some(Action::Layout(width, height)),
        }
    }

    pub fn update<M>(
        &mut self,
        model: &M,
        view: View<M, S>,
        width: f32,
        height: f32,
    ) -> Vec<Command> {
        let dom = view(&model);
        let changeset = dom::diff(&self.dom, &dom);
        self.dom = dom;

        let command = self.layout(width, height);

        let mut commands = convert(&self.dom, &self.layout, changeset);
        commands.push(command);

        commands
    }

    pub fn layout(&mut self, width: f32, height: f32) -> Command {
        let (layout, command) = {
            let ref dom = self.dom;

            let ref old_layout = self.layout;
            let new_layout = yoga(dom);
            new_layout.calculuate(width, height);

            let mut moves = vec![];
            old_layout.merge(
                &new_layout,
                |path, old, new| if old.left() != new.left() || old.top() != new.top() ||
                    old.width() != new.width() ||
                    old.height() != new.height()
                {
                    let id = path.to_string();
                    let frame = (new.left(), new.top(), new.width(), new.height());
                    moves.push((id, frame));
                },
            );

            (new_layout, Command::Move(moves))
        };

        self.layout = layout;
        command
    }
}

fn yoga<T>(node: &dom::Object<T>) -> yoga::Node {
    let mut layout = yoga::Node::new();

    // TODO: 'Flow' => Row
    // TODO: likely need to treat 'root' node differently

    match node.widget {
        dom::Widget::Stack => layout.set_direction(), // Column

        dom::Widget::Button(_) |
        dom::Widget::Label(_) |
        dom::Widget::Field(_) => {
            layout.set_margin(20.);

            layout.set_min_height(24.);
            layout.set_max_height(24.);
        }
    }

    // Traverse children, building nodes 'bottom-up'
    for (n, node) in node.children().iter().map(yoga).enumerate() {
        layout.insert(node, n as u32);
    }

    layout
}

impl tree::Vertex for yoga::Node {
    fn children(&self) -> &[Self] {
        self.children()
    }
}
