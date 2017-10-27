
use std::fmt;
use tree;

#[derive(PartialEq, Debug)]
pub enum Element {
    Text,
    Div,
    Button,
    Input,
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

    // Attributes
    pub placeholder: Option<String>,

    // Events
    pub click: Option<S>,
    pub change: Option<fn(String) -> S>,
}

impl<S> Widget<S> {
    pub fn new(element: Element) -> Self {
        Widget {
            element,

            value: None,
            placeholder: None,
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

    pub fn placeholder(mut self, text: String) -> Self {
        self.widget.placeholder = Some(text);
        self
    }

    pub fn change(mut self, change: fn(String) -> S) -> Self {
        self.widget.change = Some(change);
        self
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

#[macro_export]
macro_rules! div {
    ([$($attrs:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Div);
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, children }
    }}
}

#[macro_export]
macro_rules! button {
    ([$($attrs:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Button);
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, children }
    }}
}

#[macro_export]
macro_rules! input {
    ([$($attrs:tt)*], [$($children:tt)*]) => {{
        let widget = $crate::dom::Widget::new($crate::dom::Element::Input);
        let children = vec![ $($children)* ];

        $crate::dom::Object { widget, children }
    }}
}

pub fn text<S, T: ToString>(text: T) -> Object<S> {
    let mut widget = Widget::new(Element::Text);
    widget.value = Some(text.to_string());

    Object {
        widget,
        children: vec![],
    }
}
