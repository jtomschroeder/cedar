
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
pub type Object<S> = tree::Node<Value<S>>;

pub type Change<S> = tree::Change<Value<S>>;
pub type Changeset<S> = tree::Changeset<Value<S>>;

fn comparator<S: PartialEq>(t: &Object<S>, u: &Object<S>) -> Option<tree::Difference> {
    if t.value.0 != u.value.0 {
        Some(tree::Difference::Kind)
    } else if t.value.1 != u.value.1 {
        Some(tree::Difference::Value)
    } else {
        None
    }
}

pub fn build<S: PartialEq>(object: Object<S>) -> Changeset<S> {
    tree::diff(vec![], vec![object], comparator)
}

pub fn diff<S: PartialEq>(old: Object<S>, new: Object<S>) -> Changeset<S> {
    tree::diff(vec![old], vec![new], comparator)
}

pub trait Builder<S> {
    fn add(self, object: Self) -> Self;

    fn text(self, text: String) -> Self;
    fn click(self, action: S) -> Self;
    fn placeholder(self, text: String) -> Self;
    fn change(self, messenger: fn(String) -> S) -> Self;
}

impl<S> Builder<S> for Object<S> {
    fn add(mut self, object: Self) -> Self {
        self.children.push(object);
        self
    }

    fn text(mut self, text: String) -> Self {
        self.value.1.push(Attribute::Text(text));
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

pub fn stack<S>() -> Object<S> {
    node![(Kind::Stack, vec![])]
}

pub fn label<S>() -> Object<S> {
    node![(Kind::Label, vec![])]
}

pub fn button<S>() -> Object<S> {
    node![(Kind::Button, vec![])]
}

pub fn field<S>() -> Object<S> {
    node![(Kind::Field, vec![])]
}
