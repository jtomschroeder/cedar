
use std::fmt;
use tree;

#[derive(PartialEq, Debug)]
pub enum Element {
    Text,
    Div,
    Button,
    Input,
}

#[derive(PartialEq, Debug)]
pub enum Attribute {
    Placeholder(String),
    Style(String),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Element::*;
        write!(
            f,
            "{}",
            match self {
                &Text => "text",
                &Div => "div",
                &Button => "button",
                &Input => "input",
            }
        )
    }
}

#[derive(PartialEq)]
pub struct Widget<S> {
    pub element: Element,
    pub value: Option<String>,

    // Events
    pub click: Option<S>,
    pub change: Option<fn(String) -> S>,
}

impl<S> Widget<S> {
    pub fn new(element: Element) -> Self {
        Widget {
            element,

            value: None,
            click: None,
            change: None,
        }
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

impl<T> tree::Comparable for Object<T>
where
    T: PartialEq,
{
    fn compare(&self, other: &Self) -> Option<tree::Difference> {
        if self.widget.element != other.widget.element {
            Some(tree::Difference::Kind)
        } else if self.widget != other.widget {
            Some(tree::Difference::Value)
        } else {
            None
        }
    }
}

pub type Change = tree::Change;
pub type Changeset = tree::Changeset;

pub fn diff<S>(old: &Object<S>, new: &Object<S>) -> Changeset
where
    S: PartialEq,
{
    tree::diff(old, new)
}

/// 'Builder' methods for Object
impl<S> Object<S> {
    pub fn click(mut self, action: S) -> Self {
        self.widget.click = Some(action);
        self
    }

    pub fn change(mut self, change: fn(String) -> S) -> Self {
        self.widget.change = Some(change);
        self
    }

    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn placeholder(self, text: String) -> Self {
        self.attribute(Attribute::Placeholder(text))
    }
}

// TODO: create macro language a la JSX in React for defining DOM

// <div>
// <button></button>
// <text></text>
// <button></button>
// </div>

// or this?
// (div [(button), (text), (button)])

// or a la elm?
// view model =
//   div []
//     [ button [ onClick Decrement ] [ text "-" ]
//     , div [] [ text (toString model) ]
//     , button [ onClick Increment ] [ text "+" ]
//     ]

// maybe just put these in a cedar::prelude::*?

// Incremental TT muncher macro!?

// TODO: need to refactor the code redundancy here!

#[macro_export]
macro_rules! div {
    ([$($attributes:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Div);

        let attributes = vec![ $($attributes)* ];
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, attributes, children }
    }}
}

#[macro_export]
macro_rules! button {
    ([$($attributes:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Button);

        let attributes = vec![ $($attributes)* ];
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, attributes, children }
    }}
}

#[macro_export]
macro_rules! input {
    ([$($attributes:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Input);

        let attributes = vec![ $($attributes)* ];
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, attributes, children }
    }}
}

pub fn text<S, T: ToString>(text: T) -> Object<S> {
    let mut widget = Widget::new(Element::Text);
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


// maybe just put these in a cedar::prelude::*?

pub fn div<S>(children: Vec<Object<S>>) -> Object<S> {
    Object {
        widget: Widget::new(Element::Div),
        attributes: vec![],
        children,
    }
}

pub fn button<S>() -> Object<S> {
    Object {
        widget: Widget::new(Element::Button),
        attributes: vec![],
        children: vec![],
    }
}

pub fn input<S>() -> Object<S> {
    Object {
        widget: Widget::new(Element::Input),
        attributes: vec![],
        children: vec![],
    }
}
