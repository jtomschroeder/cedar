
use tree;

#[derive(PartialEq, Clone, Debug)]
pub struct Button<S> {
    pub text: String,
    pub click: Option<S>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Label {
    pub text: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Widget<S> {
    Stack,
    Button(Button<S>),
    Label(Label),
    Field,
}

pub type Attributes<S> = Vec<Attribute<S>>;

#[derive(Clone, Debug)]
pub struct Object<S> {
    pub widget: Widget<S>,
    pub children: Vec<Object<S>>,
}

impl<T: PartialEq> tree::Vertex for Object<T> {
    fn children(&self) -> &[Self] {
        &self.children
    }

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

/// 'Builder' methods for Object
impl<S> Object<S> {
    pub fn click(mut self, action: S) -> Self {
        match self.widget {
            Widget::Button(ref mut button) => {
                button.click = Some(action);
            }
            _ => {}
        }
        self
    }
}

pub fn stack<S>(objects: Vec<Object<S>>) -> Object<S> {
    Object {
        widget: Widget::Stack,
        children: objects,
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
        widget: Widget::Field,
        children: vec![],
    }
}
