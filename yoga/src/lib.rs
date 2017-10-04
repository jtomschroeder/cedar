
#[allow(non_camel_case_types, dead_code)]
mod sys;

#[derive(Debug)]
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
            let node = self.node;
            sys::YGNodeCalculateLayout(node, 500., 400., sys::YGDirection::YGDirectionInherit);

            for child in &self.children {
                let node = child.node;
                println!("{}", sys::YGNodeLayoutGetLeft(node));
                println!("{}", sys::YGNodeLayoutGetTop(node));
                println!("{}", sys::YGNodeLayoutGetRight(node));
                println!("{}", sys::YGNodeLayoutGetBottom(node));
                println!("{}", sys::YGNodeLayoutGetWidth(node));
                println!("{}", sys::YGNodeLayoutGetHeight(node));

                println!("");
            }
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { sys::YGNodeFree(self.node) }
    }
}
