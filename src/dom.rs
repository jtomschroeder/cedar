
use std::fmt;
use tree;

// TODO: 'text' should no longer be an attribute (now a child node)

#[derive(PartialEq)]
pub struct Button<S> {
    pub text: String,
    pub click: Option<S>,
}

#[derive(PartialEq)]
pub struct Text {
    pub text: String,
}

#[derive(PartialEq)]
pub struct Input<S> {
    pub placeholder: Option<String>,
    pub change: Option<fn(String) -> S>,
}

#[derive(PartialEq)]
pub enum Widget<S> {
    Div,
    Button(Button<S>),
    Text(Text),
    Input(Input<S>),
}

impl<S> fmt::Debug for Widget<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Widget::*;
        write!(
            f,
            "{}",
            match self {
                &Div => "Div",
                &Button(_) => "Button",
                &Text(_) => "Text",
                &Input(_) => "Input",
            }
        )
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
        if self.widget == other.widget {
            None
        } else {
            match (&self.widget, &other.widget) {
                (&Widget::Button(_), &Widget::Button(_)) |
                (&Widget::Text(_), &Widget::Text(_)) => Some(tree::Difference::Value),

                _ => Some(tree::Difference::Kind),
            }
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
        if let Widget::Button(ref mut button) = self.widget {
            button.click = Some(action);
        }
        self
    }

    pub fn placeholder(mut self, text: String) -> Self {
        if let Widget::Input(ref mut input) = self.widget {
            input.placeholder = Some(text);
        }
        self
    }

    pub fn change(mut self, change: fn(String) -> S) -> Self {
        if let Widget::Input(ref mut input) = self.widget {
            input.change = Some(change);
        }
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

pub fn div<S>(children: Vec<Object<S>>) -> Object<S> {
    Object {
        widget: Widget::Div,
        children,
    }
}

pub fn text<S>(text: String) -> Object<S> {
    Object {
        widget: Widget::Text(Text { text }),
        children: vec![],
    }
}

pub fn button<S>(text: String) -> Object<S> {
    Object {
        widget: Widget::Button(Button { text, click: None }),
        children: vec![],
    }
}

pub fn input<S>() -> Object<S> {
    Object {
        widget: Widget::Input(Input {
            placeholder: None,
            change: None,
        }),

        children: vec![],
    }
}
