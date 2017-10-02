
use tree;

// pub struct Button<S> {
//     text: String,
//     click: S,
// }

pub type Attributes<S> = Vec<Attribute<S>>;
pub type Value<S> = (Kind, Attributes<S>);

#[derive(Clone, Debug)]
pub struct Object<S> {
    pub value: Value<S>,
    pub children: Vec<Object<S>>,
}

impl<T: PartialEq> tree::Vertex for Object<T> {
    fn children(&self) -> &[Self] {
        &self.children
    }

    fn compare(&self, other: &Self) -> Option<tree::Difference> {
        if self.value.0 != other.value.0 {
            Some(tree::Difference::Kind)
        } else if self.value.1 != other.value.1 {
            Some(tree::Difference::Value)
        } else {
            None
        }
    }
}

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

pub type Change = tree::Change;
pub type Changeset = tree::Changeset;

pub fn diff<S: PartialEq>(old: &Object<S>, new: &Object<S>) -> Changeset {
    tree::diff(old, new)
}

pub trait Builder<S> {
    fn text(self, text: String) -> Self;
    fn click(self, action: S) -> Self;
    fn placeholder(self, text: String) -> Self;
    fn change(self, messenger: fn(String) -> S) -> Self;
}

impl<S> Builder<S> for Object<S> {
    fn text(mut self, text: String) -> Self {
        self.value.1.push(Attribute::Text(text.into()));
        self
    }
    fn click(mut self, action: S) -> Self {
        self.value.1.push(Attribute::Click(action));
        self
    }
    fn placeholder(mut self, text: String) -> Self {
        self.value.1.push(Attribute::Placeholder(text));
        self
    }
    fn change(mut self, messenger: fn(String) -> S) -> Self {
        self.value.1.push(Attribute::Change(messenger));
        self
    }
}

pub fn stack<S>(objects: Vec<Object<S>>) -> Object<S> {
    Object {
        value: (Kind::Stack, vec![]),
        children: objects,
    }
}

pub fn label<S>() -> Object<S> {
    Object {
        value: (Kind::Label, vec![]),
        children: vec![],
    }
}

pub fn button<S>() -> Object<S> {
    Object {
        value: (Kind::Button, vec![]),
        children: vec![],
    }
}

pub fn field<S>() -> Object<S> {
    Object {
        value: (Kind::Field, vec![]),
        children: vec![],
    }
}
