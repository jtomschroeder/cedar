
use dom;
use std::fmt::Debug;

use super::{Window, Label, Container, Button, TextField};
use super::widget::Widget;

use stream::Stream;
use atomic_box::AtomicBox;

use layout::yoga::*;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Vertex<S> {
    widget: AtomicBox<Box<Widget<S>>>,
    children: Vec<Vertex<S>>,
    layout: AtomicPtr<YGNode>,
}

type Tree<S> = Vec<Vertex<S>>;

impl<S> Vertex<S> {
    fn layout(&mut self, width: f32, height: f32) {
        unsafe {
            let node = self.layout.load(Ordering::Relaxed);
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

fn create<S: Clone + 'static>(
    stream: Stream<S>,
    node: dom::Object<S>,
    layout: YGNodeRef,
) -> Vertex<S> {
    let (kind, attributes) = (node.value.0, node.value.1);
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

                YGNodeInsertChild(layout, node, 0);
                node
            };

            let child = create(stream.clone(), child, node);
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

// TODO: use `removeFromSuperview()` to 'delete' nodes

fn patch<S: Debug>(tree: &mut Tree<S>, (mut path, op): dom::Change<S>) {
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
        patch(&mut tree[location.index].children, (path, op));
    }
}

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

    let (_window, mut content_view) = Window::new("cedar");

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

    let vertex = create(stream.clone(), node.clone(), root);
    content_view.add(&vertex.widget);

    let mut tree = vec![vertex];

    app.run(move || {
        let mut model = model;
        let mut node = node;

        loop {
            // trigger layout of `tree` and update widgets
            tree[0].layout(500., 400.);

            let message = stream.pop();

            // println!("msg: {:?}", message);

            model = update(model, message);

            let old = node;
            node = view(&model);

            // println!("node: {:?}", new);

            let changeset = dom::diff(old, node.clone());

            // println!("diff: {:?}", changeset);

            for change in changeset.into_iter() {
                patch(&mut tree, change);
            }
        }
    })
}
