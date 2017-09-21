
use dom;
use std::fmt::Debug;

use super::{Window, Label, Container, Button, TextField};
use super::widget::Widget;

use stream::Stream;
use atomic_box::AtomicBox;

use layout::yoga::*;
use std::sync::atomic::{AtomicPtr, Ordering};

mod vertex {
    use std::fmt;
    use std::sync::atomic::{AtomicPtr, Ordering};

    use atomic_box::AtomicBox;
    use layout::yoga::*;
    use dom;
    use super::{Window, Label, Container, Button, TextField};
    use cacao::widget::Widget;
    use stream::Stream;

    pub struct Vertex<S> {
        pub widget: AtomicBox<Box<Widget<S>>>,
        pub children: Vec<Vertex<S>>,
        layout: AtomicPtr<YGNode>,
    }

    pub type Tree<S> = Vec<Vertex<S>>;

    impl<S> Vertex<S> {
        pub fn layout(&mut self, width: f32, height: f32) {
            unsafe {
                let node = self.layout.load(Ordering::Relaxed);

                YGNodeStyleSetWidth(node, width);
                YGNodeStyleSetHeight(node, height);

                YGNodeCalculateLayout(node, width, height, YGDirection::YGDirectionInherit);
            }

            self.resize();
        }

        fn resize(&mut self) {
            unsafe {
                let node = self.layout.load(Ordering::Relaxed);

                let left = YGNodeLayoutGetLeft(node);
                let top = YGNodeLayoutGetTop(node);
                // let right = YGNodeLayoutGetRight(node);
                // let bottom = YGNodeLayoutGetBottom(node);
                let width = YGNodeLayoutGetWidth(node);
                let height = YGNodeLayoutGetHeight(node);

                self.widget.layout(
                    left as f64,
                    top as f64,
                    width as f64,
                    height as f64,
                );
            }

            // resize children
            for v in &mut self.children {
                v.resize();
            }
        }
    }

    impl<S: Clone + 'static> Vertex<S> {
        pub fn create(stream: Stream<S>, node: dom::Object<S>, layout: YGNodeRef) -> Vertex<S> {
            let (kind, attributes) = node.value;
            let mut widget: Box<Widget<S>> = match kind {
                dom::Kind::Label => Box::new(Label::new()),
                dom::Kind::Button => Box::new(Button::new(stream.clone())),
                dom::Kind::Stack => Box::new(Container::new()), // TODO: set flex direction to 'column' for stacking
                dom::Kind::Field => Box::new(TextField::new(stream.clone())),
            };

            widget.update(attributes);

            let children = node.children
                .into_iter()
                .map(|child| {
                    let node = unsafe {
                        let node = YGNodeNew();

                        YGNodeStyleSetFlexGrow(node, 1.);

                        // TODO: define 'default' size for widget when creating layout node

                        // YGNodeStyleSetWidth(node, 50.);
                        // YGNodeStyleSetHeight(node, 50.);

                        YGNodeInsertChild(layout, node, 0);
                        node
                    };

                    let child = Vertex::create(stream.clone(), child, node);
                    widget.add(&child.widget);
                    child
                })
                .collect();

            Vertex {
                widget: AtomicBox::new(widget),
                children,
                layout: AtomicPtr::new(layout),
            }
        }
    }

    // TODO: use `removeFromSuperview()` to 'delete' nodes

    impl<S: fmt::Debug> Vertex<S> {
        pub fn patch(tree: &mut Tree<S>, (mut path, op): dom::Change<S>) {
            if path.is_empty() {
                return;
            }

            let location = path.remove(0);
            if path.is_empty() {
                let widget = &mut tree[location.index].widget;

                use tree::Operation::*;
                match op {
                    Update((_, attrs)) => widget.update(attrs),
                    op => panic!("Not yet implemented! {:?}", op),
                }
            } else {
                Self::patch(&mut tree[location.index].children, (path, op));
            }
        }
    }
}

use self::vertex::{Vertex, Tree};

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    let app = super::Application::new(); // TODO: enforce `app` created first

    let stream = Stream::new();

    // TODO: allow custom app name

    let (window, mut content_view) = Window::new("cedar");

    let node = view(&model);

    let root = unsafe {
        let root = YGNodeNew();

        // TODO: Should match window/content-view frame size!
        YGNodeStyleSetWidth(root, 500.);
        YGNodeStyleSetHeight(root, 400.);

        // Stack => Column Direction
        YGNodeStyleSetFlexDirection(root, YGFlexDirection::YGFlexDirectionColumn);

        // YGNodeStyleSetPadding(root, YGEdge::YGEdgeAll, 20.);

        root
    };

    let vertex = Vertex::create(stream.clone(), node.clone(), root);
    content_view.add(&vertex.widget);

    let mut tree = vec![vertex];

    app.run(move || {
        let mut model = model;
        let mut node = node;

        loop {
            {
                let root = &mut tree[0];

                let frame = window.frame();

                // TODO: trigger event on `stream` with new window size

                // trigger layout of `tree` and update widgets
                root.layout(frame.size.width as f32, frame.size.height as f32);
            }

            let message = stream.pop();

            // println!("msg: {:?}", message);

            model = update(model, message);

            let old = node;
            node = view(&model);

            // println!("node: {:?}", new);

            let changeset = dom::diff(old, node.clone());

            // println!("diff: {:?}", changeset);

            for change in changeset.into_iter() {
                Vertex::patch(&mut tree, change);
            }
        }
    })
}
