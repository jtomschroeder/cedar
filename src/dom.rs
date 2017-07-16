
use tree;

#[derive(PartialEq, Clone, Debug)]
pub enum Kind {
    Stack,
    Button,
    Label,
    Field,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Attribute<S> {
    Text(String),
    Click(S),
    Placeholder(String),
    Change(fn(String) -> S),
}

pub type Attributes<S> = Vec<Attribute<S>>;

pub type Value<S> = (Kind, Attributes<S>);
pub type Node<S> = tree::Node<Value<S>>;

pub type Change<S> = tree::Change<Value<S>>;
pub type Changeset<S> = tree::Changeset<Value<S>>;

pub fn diff<S: PartialEq>(old: Node<S>, new: Node<S>) -> Changeset<S> {
    fn comparator<S: PartialEq>(t: &Node<S>, u: &Node<S>) -> Option<tree::Difference> {
        if t.value.0 != u.value.0 {
            Some(tree::Difference::Kind)
        } else if t.value.1 != u.value.1 {
            Some(tree::Difference::Value)
        } else {
            None
        }
    }

    tree::diff(vec![old], vec![new], comparator)
}

pub struct Object<S> {
    node: Node<S>,
}

impl<S> Object<S> {
    pub fn add(mut self, object: Self) -> Self {
        self.node.children.push(object.node);
        self
    }

    pub fn create(self) -> Node<S> {
        self.node
    }

    pub fn text(mut self, text: String) -> Self {
        self.node.value.1.push(Attribute::Text(text));
        self
    }
    pub fn click(mut self, action: S) -> Self {
        self.node.value.1.push(Attribute::Click(action));
        self
    }
    pub fn placeholder(mut self, text: String) -> Self {
        self.node.value.1.push(Attribute::Placeholder(text));
        self
    }
    pub fn change(mut self, messenger: fn(String) -> S) -> Self {
        self.node.value.1.push(Attribute::Change(messenger));
        self
    }
}

pub fn stack<S>() -> Object<S> {
    Object { node: node![(Kind::Stack, vec![])] }
}

pub fn label<S>() -> Object<S> {
    Object { node: node![(Kind::Label, vec![])] }
}

pub fn button<S>() -> Object<S> {
    Object { node: node![(Kind::Button, vec![])] }
}

pub fn field<S>() -> Object<S> {
    Object { node: node![(Kind::Field, vec![])] }
}
