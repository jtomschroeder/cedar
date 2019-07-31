use crate::tree;
use std::fmt;
use std::collections::HashMap;

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
        write!(
            f,
            "{}",
            match self {
                Attribute::String {
                    ref name,
                    ref value,
                } => format!("{}:{}", name, value),
                Attribute::Click(_) => "click".to_string(),
                Attribute::Input(_) => "input".to_string(),
                Attribute::Keydown(_) => "keydown".to_string(),
            }
        )
    }
}

impl<S> PartialEq for Attribute<S> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: implement this!!

        false
    }
}

pub struct Properties<S> {
    pub attributes: HashMap<String, Attribute<S>>,
    pub children: Vec<Object<S>>,
}

impl<S> Default for Properties<S> {
    fn default() -> Self {
        Properties {
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}

pub struct Object<S> {
    element: Element,
    value: Option<String>,
    pub props: Properties<S>,
}

impl<S> Object<S> {
    pub fn new(element: &str) -> Self {
        Object {
            element: element.into(),
            value: None,
            props: Properties::default(),
        }
    }

    pub fn create(element: &str, props: Properties<S>) -> Self {
        Object {
            element: element.into(),
            value: None,
            props,
        }
    }

    pub fn text(text: impl ToString) -> Self {
        Object {
            element: "text".into(),
            value: Some(text.to_string()),
            props: Properties::default(),
        }
    }

    pub fn is_text(&self) -> bool {
        self.element == "text"
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn push(mut self, pushed: impl Pushable<S>) -> Self {
        pushed.pushed(&mut self.props.children);
        self
    }
}

impl<S> fmt::Debug for Object<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("element", &self.element)
            .field("value", &self.value)
            .field("attributes", &self.props.attributes)
            .field("children", &self.props.children)
            .finish()
    }
}

impl<T> tree::Vertex for Object<T> {
    fn children(&self) -> &[Self] {
        &self.props.children
    }
}

impl<T: PartialEq> tree::Comparable for Object<T> {
    fn compare(&self, other: &Self) -> Option<tree::Difference> {
        if self.element != other.element {
            Some(tree::Difference::Kind)
        } else if self.value != other.value || self.props.attributes != other.props.attributes {
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
    fn pushed(self, objects: &mut Vec<Object<S>>);
}

impl<S> Pushable<S> for Object<S> {
    fn pushed(self, objects: &mut Vec<Object<S>>) {
        objects.push(self);
    }
}

impl<'s, S> Pushable<S> for Vec<Object<S>> {
    fn pushed(self, objects: &mut Vec<Object<S>>) {
        objects.extend(self);
    }
}

impl<'s, S> Pushable<S> for &'s str {
    fn pushed(self, objects: &mut Vec<Object<S>>) {
        objects.push(text(self));
    }
}

impl<S> Pushable<S> for String {
    fn pushed(self, objects: &mut Vec<Object<S>>) {
        objects.push(text(self));
    }
}

pub fn text<S>(text: impl ToString) -> Object<S> {
    Object::text(text)
}
