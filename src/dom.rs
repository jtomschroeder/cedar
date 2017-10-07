
use tree;

#[derive(PartialEq, Debug)]
pub struct Button<S> {
    pub text: String,
    pub click: Option<S>,
}

#[derive(PartialEq, Debug)]
pub struct Label {
    pub text: String,
}

#[derive(PartialEq, Debug)]
pub struct Field<S> {
    pub placeholder: Option<String>,
    pub change: Option<fn(String) -> S>,
}

#[derive(PartialEq, Debug)]
pub enum Widget<S> {
    Stack,
    Button(Button<S>),
    Label(Label),
    Field(Field<S>),
}

#[derive(Debug)]
pub struct Object<S> {
    pub widget: Widget<S>,
    pub children: Vec<Object<S>>,
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
                (&Widget::Label(_), &Widget::Label(_)) => Some(tree::Difference::Value),

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
        if let Widget::Field(ref mut field) = self.widget {
            field.placeholder = Some(text);
        }
        self
    }

    pub fn change(mut self, change: fn(String) -> S) -> Self {
        if let Widget::Field(ref mut field) = self.widget {
            field.change = Some(change);
        }
        self
    }
}

// TODO: create macro language a la JSX in React for defining DOM

// <stack>
// <button></button>
// <label></button>
// <button></button>
// </stack>

// or this?
// (stack [(button), (label), (button)])

// or a la elm?
// view model =
//   div []
//     [ button [ onClick Decrement ] [ text "-" ]
//     , div [] [ text (toString model) ]
//     , button [ onClick Increment ] [ text "+" ]
//     ]

pub fn stack<S>(children: Vec<Object<S>>) -> Object<S> {
    Object {
        widget: Widget::Stack,
        children,
    }
}

pub fn label<S>(text: String) -> Object<S> {
    Object {
        widget: Widget::Label(Label { text }),
        children: vec![],
    }
}

pub fn button<S>(text: String) -> Object<S> {
    Object {
        widget: Widget::Button(Button { text, click: None }),
        children: vec![],
    }
}

pub fn field<S>() -> Object<S> {
    Object {
        widget: Widget::Field(Field {
            placeholder: None,
            change: None,
        }),

        children: vec![],
    }
}
