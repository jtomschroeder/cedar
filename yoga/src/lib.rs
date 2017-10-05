
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
            node: unsafe { sys::YGNodeNew() },
            children: vec![],
        }
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn insert(&mut self, child: Node, index: u32) {
        unsafe {
            sys::YGNodeStyleSetFlexGrow(child.node, 1.);
            sys::YGNodeInsertChild(self.node, child.node, index);
        }

        // TODO: use `index` here?
        self.children.push(child);
    }

    pub fn calculuate(&self) {
        unsafe {
            sys::YGNodeCalculateLayout(self.node, 500., 400., sys::YGDirection::YGDirectionInherit);
        }
    }

    fn left(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetLeft(self.node) }
    }
    fn top(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetTop(self.node) }
    }
    fn right(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetRight(self.node) }
    }
    fn bottom(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetBottom(self.node) }
    }
    fn width(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetWidth(self.node) }
    }
    fn height(&self) -> f32 {
        unsafe { sys::YGNodeLayoutGetHeight(self.node) }
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
