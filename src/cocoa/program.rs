
use dom;
use std::fmt::Debug;

use super::{Window, Label, Stack, Button, TextField};
use super::widget::Widget;

use stream::Stream;
use atomic_box::AtomicBox;

use layout::yoga::*;
use std::sync::atomic::{AtomicPtr, Ordering};

// TODO: add Yoga Node into vertex

struct Vertex<S> {
    widget: AtomicBox<Box<Widget<S>>>,
    children: Vec<Vertex<S>>,
    layout: AtomicPtr<YGNode>,
}

type Tree<S> = Vec<Vertex<S>>;

fn create<S: Clone + 'static>(
    stream: Stream<S>,
    node: dom::Object<S>,
    layout: YGNodeRef,
) -> Vertex<S> {
    let (kind, attributes) = (node.value.0, node.value.1);
    let mut widget: Box<Widget<S>> = match kind {
        dom::Kind::Label => Box::new(Label::new()),
        dom::Kind::Button => Box::new(Button::new(stream.clone())),
        dom::Kind::Stack => Box::new(Stack::new()),
        dom::Kind::Field => Box::new(TextField::new(stream.clone())),
    };

    widget.update(attributes);

    let children = node.children
        .into_iter()
        .map(|child| {
            let node = unsafe {
                let node = YGNodeNew();
                // YGNodeStyleSetHeight(node, 60.);
                // YGNodeStyleSetWidth(node, 80.);
                YGNodeStyleSetFlexGrow(node, 1.);

                YGNodeInsertChild(layout, node, YGNodeGetChildCount(layout));
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

    let (_window, mut stack) = Window::new("cedar");

    let node = view(&model);

    let root = unsafe {
        let root = YGNodeNew();

        // Should match window size
        YGNodeStyleSetWidth(root, 500.);
        YGNodeStyleSetHeight(root, 400.);

        // Stack => Column Direction
        YGNodeStyleSetFlexDirection(root, YGFlexDirection::YGFlexDirectionColumn);

        YGNodeStyleSetPadding(root, YGEdge::YGEdgeAll, 20.);

        root
    };

    let vertex = create(stream.clone(), node.clone(), root);
    stack.add(&vertex.widget);

    let mut tree = vec![vertex];

    app.run(move || {
        let mut model = model;
        let mut node = node;

        loop {
            unsafe {
                let root = tree[0].layout.load(Ordering::Relaxed);
                YGNodeCalculateLayout(root, 500., 400., YGDirection::YGDirectionInherit);

                let node = tree[0].children[0].layout.load(Ordering::Relaxed);

                println!("left: {}", YGNodeLayoutGetLeft(node));
                println!("top: {}", YGNodeLayoutGetTop(node));
                println!("right: {}", YGNodeLayoutGetRight(node));
                println!("bottom: {}", YGNodeLayoutGetBottom(node));
                println!("width: {}", YGNodeLayoutGetWidth(node));
                println!("height: {}", YGNodeLayoutGetHeight(node));

                println!("");

                let node = tree[0].children[1].layout.load(Ordering::Relaxed);

                println!("left: {}", YGNodeLayoutGetLeft(node));
                println!("top: {}", YGNodeLayoutGetTop(node));
                println!("right: {}", YGNodeLayoutGetRight(node));
                println!("bottom: {}", YGNodeLayoutGetBottom(node));
                println!("width: {}", YGNodeLayoutGetWidth(node));
                println!("height: {}", YGNodeLayoutGetHeight(node));

                println!("");

                let node = tree[0].children[2].layout.load(Ordering::Relaxed);

                println!("left: {}", YGNodeLayoutGetLeft(node));
                println!("top: {}", YGNodeLayoutGetTop(node));
                println!("right: {}", YGNodeLayoutGetRight(node));
                println!("bottom: {}", YGNodeLayoutGetBottom(node));
                println!("width: {}", YGNodeLayoutGetWidth(node));
                println!("height: {}", YGNodeLayoutGetHeight(node));

                println!("");
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
                patch(&mut tree, change);
            }

            // TODO: trigger layout of `tree` and update widgets
        }
    })
}
