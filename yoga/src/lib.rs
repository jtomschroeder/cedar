
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

                // TODO: remove these! (temporary for prototyping)
                sys::YGNodeStyleSetFlexGrow(node, 1.);
                sys::YGNodeStyleSetPadding(node, sys::YGEdge::YGEdgeAll, 20.);

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

    pub fn calculuate(&self) {
        unsafe {
            sys::YGNodeCalculateLayout(self.node, 500., 500., sys::YGDirection::YGDirectionInherit);
        }
    }

    pub fn set_direction(&mut self) {
        // TODO: should NOT be hardcoded!!
        unsafe {
            // YGFlexDirectionColumnReverse
            sys::YGNodeStyleSetFlexDirection(self.node, sys::YGFlexDirection::YGFlexDirectionColumn)
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
