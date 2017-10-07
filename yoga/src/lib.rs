
#[allow(non_camel_case_types, dead_code)]
mod sys;

use std::fmt;

pub struct Node {
    node: sys::YGNodeRef,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            node: unsafe {
                let node = sys::YGNodeNew();

                // TODO: remove this? (temporary for prototyping)
                sys::YGNodeStyleSetFlexGrow(node, 1.);

                node
            },
            children: vec![],
        }
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn insert(&mut self, child: Node, index: u32) {
        unsafe { sys::YGNodeInsertChild(self.node, child.node, index) };

        // TODO: use `index` here?
        self.children.push(child);
    }

    pub fn calculuate(&self, width: f32, height: f32) {
        unsafe {
            sys::YGNodeCalculateLayout(
                self.node,
                width,
                height,
                sys::YGDirection::YGDirectionInherit,
            );
        }
    }

    pub fn set_direction(&mut self) {
        // TODO: should NOT be hardcoded!!
        unsafe {
            // YGFlexDirectionColumnReverse
            let direction = sys::YGFlexDirection::YGFlexDirectionColumn;
            sys::YGNodeStyleSetFlexDirection(self.node, direction)
        };
    }

    pub fn left(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetLeft(self.node) }
    }
    pub fn top(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetTop(self.node) }
    }
    pub fn right(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetRight(self.node) }
    }
    pub fn bottom(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetBottom(self.node) }
    }
    pub fn width(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetWidth(self.node) }
    }
    pub fn height(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetHeight(self.node) }
    }

    pub fn set_margin(&mut self, margin: f32) {
        unsafe { sys::YGNodeStyleSetMargin(self.node, sys::YGEdge::YGEdgeAll, margin) };
    }

    pub fn set_min_height(&mut self, height: f32) {
        unsafe { sys::YGNodeStyleSetMinHeight(self.node, height) }
    }
    pub fn set_max_height(&mut self, height: f32) {
        unsafe { sys::YGNodeStyleSetMaxHeight(self.node, height) }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { sys::YGNodeFree(self.node) }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("left", &self.left())
            .field("top", &self.top())
            .field("right", &self.right())
            .field("bottom", &self.bottom())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
