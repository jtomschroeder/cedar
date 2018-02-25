use std::fmt::{self, Display};
use tree;

pub type Element = String;

macro_rules! element {
    ($name:ident) => {
        pub fn $name<S>() -> Object<S> {
            Object::new(stringify!($name))
        }
    }
}

element!(div);
element!(button);
element!(input);
element!(label);
element!(section);
element!(header);
element!(footer);
element!(span);
element!(strong);
element!(h1);
element!(ul);
element!(li);
element!(a);
element!(p);

// TODO: {hidden, autofocus, checked} should be a Boolean in JS front-end!

#[derive(PartialEq, Debug)]
pub enum Attribute {
    Placeholder(String),
    Class(String),
    Style(String),
    Hidden(bool),
    Other(String, String),
}

impl Attribute {
    pub fn raw(&self) -> (String, String) {
        let (name, value) = match self {
            &Attribute::Placeholder(ref p) => ("placeholder", p.as_str()),
            &Attribute::Class(ref c) => ("className", c.as_str()),
            &Attribute::Style(ref s) => ("style", s.as_str()),
            &Attribute::Hidden(hidden) => ("hidden", if hidden { "true" } else { "false" }),
            &Attribute::Other(ref name, ref value) => (name.as_str(), value.as_str()),
        };

        (name.into(), value.into())
    }
}

macro_rules! attribute {
    ($name:ident => $attr:ident) => {
        pub fn $name<T: ToString>(self, s: T) -> Self {
            self.attribute(Attribute::$attr(s.to_string()))
        }
    }
}

pub struct Widget<S> {
    element: Element,
    pub value: Option<String>,

    // Events
    pub click: Option<S>,
    pub input: Option<Box<Fn(String) -> S>>,
    pub keydown: Option<Box<Fn(u32) -> Option<S>>>,
}

impl<S> PartialEq for Widget<S> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element && self.value == other.value
    }
}

impl<S> Widget<S> {
    pub fn new(element: Element) -> Self {
        Widget {
            element,

            value: None,

            click: None,
            input: None,
            keydown: None,
        }
    }

    pub fn is_text(&self) -> bool {
        self.element == "text"
    }

    pub fn element(&self) -> String {
        self.element.clone()
    }
}

impl<S> fmt::Debug for Widget<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.element)
    }
}

pub struct Object<S> {
    pub widget: Widget<S>,
    pub attributes: Vec<Attribute>,
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
}

impl<S> fmt::Debug for Object<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("widget", &self.widget)
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

/// Object: Actions
impl<S> Object<S> {
    pub fn click(mut self, action: S) -> Self {
        self.widget.click = Some(action);
        self
    }

    pub fn input<F>(mut self, input: F) -> Self
    where
        F: Fn(String) -> S + 'static,
    {
        self.widget.input = Some(Box::new(input));
        self
    }

    pub fn keydown<F>(mut self, keydown: F) -> Self
    where
        F: Fn(u32) -> Option<S> + 'static,
    {
        self.widget.keydown = Some(Box::new(keydown));
        self
    }
}

/// Object: Attributes
impl<S> Object<S> {
    pub fn attr(self, name: &str, value: &str) -> Self {
        let name = match name {
            "class" => "className",
            // "value" => "nodeValue",
            _ => name,
        };

        self.attribute(Attribute::Other(name.into(), value.into()))
    }

    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    attribute!(placeholder => Placeholder);
    attribute!(class => Class);

    pub fn style<T: Display, U: Display>(self, mut attrs: Vec<(T, U)>) -> Self {
        let style = attrs
            .drain(..)
            .map(|(name, value)| format!("{}: {}; ", name, value))
            .fold(String::new(), |mut style, s| {
                style += &s;
                style
            });

        self.attribute(Attribute::Style(style))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.attribute(Attribute::Hidden(value))
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
}

pub fn text<S, T: ToString>(text: T) -> Object<S> {
    let mut widget = Widget::new("text".into());
    widget.value = Some(text.to_string());

    Object {
        widget,
        attributes: vec![],
        children: vec![],
    }
}

// Attributes

pub fn placeholder<T: ToString>(text: T) -> Attribute {
    Attribute::Placeholder(text.to_string())
}

pub fn style(mut attrs: Vec<(String, String)>) -> Attribute {
    let style = attrs
        .drain(..)
        .map(|(name, value)| format!("{}: {}; ", name, value))
        .fold(String::new(), |mut style, s| {
            style += &s;
            style
        });

    Attribute::Style(style)
}

#[macro_export]
macro_rules! style {
    ($(($name:expr, $value:expr)),*) => {{
        let attrs = vec![$(($name.into(), $value.into())),*];
        $crate::dom::style(attrs)
    }}
}
