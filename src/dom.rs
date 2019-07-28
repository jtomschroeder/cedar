use crate::tree;
use std::fmt;

pub type Element = String;

pub enum Attribute<S> {
    String { name: String, value: String },
    Click(S),
    Input(Box<dyn Fn(String) -> S>),
    Keydown(Box<dyn Fn(u32) -> Option<S>>),
}

impl<S> Attribute<S> {
    pub fn input(input: impl Fn(String) -> S + 'static) -> Self {
        Attribute::Input(Box::new(input))
    }

    pub fn keydown(keydown: impl Fn(u32) -> Option<S> + 'static) -> Self {
        Attribute::Keydown(Box::new(keydown))
    }

    pub fn raw(&self) -> Option<(String, String)> {
        match self {
            Attribute::String {
                ref name,
                ref value,
            } => Some((name.clone(), value.clone())),
            _ => None,
        }
    }
}

impl<S> fmt::Debug for Attribute<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "attr?",) // TODO!
    }
}

impl<S> PartialEq for Attribute<S> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: implement this!!

        false
    }
}

// TODO: make `attributes` a hash-map instead of vector

pub struct Object<S> {
    pub element: Element,
    pub value: Option<String>,
    pub attributes: Vec<Attribute<S>>,
    pub children: Vec<Object<S>>,
}

/// Object: Actions
impl<S> Object<S> {
    pub fn new(element: &str) -> Self {
        Object {
            element: element.into(),
            value: None,
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn text(text: impl ToString) -> Self {
        Object {
            element: "text".into(),
            value: Some(text.to_string()),
            attributes: vec![],
            children: vec![],
        }
    }
    pub fn is_text(&self) -> bool {
        self.element == "text"
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn attr(mut self, attr: Attribute<S>) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn add(mut self, child: Object<S>) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<Object<S>>) -> Self {
        self.children.extend(children.into_iter());
        self
    }

    pub fn push(mut self, pushed: impl Pushable<S>) -> Self {
        pushed.pushed(&mut self);
        self
    }
}

impl<S> fmt::Debug for Object<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("element", &self.element)
            .field("value", &self.value)
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .finish()
    }
}

impl<T> tree::Vertex for Object<T> {
    fn children(&self) -> &[Self] {
        &self.children
    }
}

impl<T: PartialEq> tree::Comparable for Object<T> {
    fn compare(&self, other: &Self) -> Option<tree::Difference> {
        if self.element != other.element {
            Some(tree::Difference::Kind)
        } else if self.value != other.value || self.attributes != other.attributes {
            Some(tree::Difference::Value)
        } else {
            None
        }
    }
}

pub type Change = tree::Change;
pub type Changeset = tree::Changeset;

pub fn diff<S: PartialEq>(old: &Object<S>, new: &Object<S>) -> Changeset {
    tree::diff(old, new)
}

pub trait Pushable<S> {
    fn pushed(self, object: &mut Object<S>);
}

impl<S> Pushable<S> for Object<S> {
    fn pushed(self, object: &mut Object<S>) {
        object.children.push(self);
    }
}

impl<'s, S> Pushable<S> for Vec<Object<S>> {
    fn pushed(self, object: &mut Object<S>) {
        object.children.extend(self);
    }
}

impl<'s, S> Pushable<S> for &'s str {
    fn pushed(self, object: &mut Object<S>) {
        object.children.push(text(self));
    }
}

impl<S> Pushable<S> for String {
    fn pushed(self, object: &mut Object<S>) {
        object.children.push(text(self));
    }
}

pub fn text<S>(text: impl ToString) -> Object<S> {
    Object::text(text)
}
