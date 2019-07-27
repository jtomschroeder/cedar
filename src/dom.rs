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

#[derive(PartialEq, Debug)]
pub struct Widget {
    element: Element,
    pub value: Option<String>,
}

impl Widget {
    pub fn new(element: Element) -> Self {
        Widget {
            element,
            value: None,
        }
    }

    pub fn new_with_value(element: Element, value: String) -> Self {
        Widget {
            element,
            value: Some(value),
        }
    }

    pub fn is_text(&self) -> bool {
        self.element == "text"
    }

    pub fn element(&self) -> String {
        self.element.clone()
    }
}

pub struct Object<S> {
    pub widget: Widget,
    pub attributes: Vec<Attribute<S>>,
    pub children: Vec<Object<S>>,
}

/// Object: Actions
impl<S> Object<S> {
    pub fn new(widget: &str) -> Self {
        Object {
            widget: Widget::new(widget.into()),
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn from_widget(widget: Widget) -> Self {
        Object {
            widget,
            attributes: vec![],
            children: vec![],
        }
    }
}

impl<S> fmt::Debug for Object<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("widget", &self.widget)
            .field("attributes", &self.attributes)
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
        if self.widget.element != other.widget.element {
            Some(tree::Difference::Kind)
        } else if self.widget != other.widget || self.attributes != other.attributes {
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

///// Object: Actions
//impl<S> Object<S> {
//    pub fn click(mut self, action: S) -> Self {
//        self.widget.click = Some(action);
//        self
//    }
//
//    pub fn input<F>(mut self, input: F) -> Self
//    where
//        F: Fn(String) -> S + 'static,
//    {
//        self.widget.input = Some(Box::new(input));
//        self
//    }
//
//    pub fn keydown<F>(mut self, keydown: F) -> Self
//    where
//        F: Fn(u32) -> Option<S> + 'static,
//    {
//        self.widget.keydown = Some(Box::new(keydown));
//        self
//    }
//}

/// Object: Attributes
impl<S> Object<S> {
    pub fn attr(mut self, attr: Attribute<S>) -> Self {
        self.attributes.push(attr);
        self
    }
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

/// Object: Children
impl<S> Object<S> {
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

    // pub fn value(mut self, value: impl ToString) -> Self {
    //     self.widget.set_value(value);
    //     self
    // }
}

pub fn text<S, T: ToString>(text: T) -> Object<S> {
    let widget = Widget::new_with_value("text".into(), text.to_string());
    Object::from_widget(widget)
}
