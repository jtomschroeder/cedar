
#[allow(non_camel_case_types, dead_code)]
mod yoga;

#[derive(Debug)]
pub struct Node {
    node: yoga::YGNodeRef,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            node: unsafe { yoga::YGNodeNew() },
            children: vec![],
        }
    }

    pub fn insert(&mut self, child: Node, index: u32) {
        unsafe {
            yoga::YGNodeStyleSetFlexGrow(child.node, 1.);
            yoga::YGNodeInsertChild(self.node, child.node, index);
        }

        // TODO: use `index` here?
        self.children.push(child);
    }

    pub fn calculuate(&self) {
        unsafe {
            let node = self.node;
            yoga::YGNodeCalculateLayout(node, 500., 400., yoga::YGDirection::YGDirectionInherit);

            for child in &self.children {
                let node = child.node;
                println!("{}", yoga::YGNodeLayoutGetLeft(node));
                println!("{}", yoga::YGNodeLayoutGetTop(node));
                println!("{}", yoga::YGNodeLayoutGetRight(node));
                println!("{}", yoga::YGNodeLayoutGetBottom(node));
                println!("{}", yoga::YGNodeLayoutGetWidth(node));
                println!("{}", yoga::YGNodeLayoutGetHeight(node));

                println!("");
            }
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { yoga::YGNodeFree(self.node) }
    }
}
