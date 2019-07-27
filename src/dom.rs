use crate::tree;
use std::fmt;

pub type Element = String;

pub enum NewAttribute<S> {
    String { name: String, value: String },
    Click(S),
    Input(Box<dyn Fn(String) -> S>),
    Keydown(Box<dyn Fn(u32) -> Option<S>>),
}

impl<S> NewAttribute<S> {
    pub fn input(input: impl  Fn(String) -> S + 'static) -> Self {
        NewAttribute::Input(Box::new(input))
    }
}

impl<S> fmt::Debug for NewAttribute<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "attr?",)
    }
}

impl<S> PartialEq for NewAttribute<S> {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(PartialEq, Debug)]
pub struct Attribute(String, String);

impl Attribute {
    pub fn raw(&self) -> (String, String) {
        match self {
            &Attribute(ref name, ref value) => (name.clone(), value.clone()),
        }
    }
}

//trait IntoAttribute {
//
//}

pub struct Widget<S> {
    element: Element,
    pub value: Option<String>,

    // Events
    pub click: Option<S>,
    pub input: Option<Box<dyn Fn(String) -> S>>,
    pub keydown: Option<Box<dyn Fn(u32) -> Option<S>>>,
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

    pub fn new_with_value(element: Element, value: String) -> Self {
        Widget {
            element,

            value: Some(value),

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

    // pub fn set_value(&mut self, value: impl ToString) {
    //     self.value = Some(value.to_string());
    // }
}

impl<S> fmt::Debug for Widget<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.element,
            self.value.clone().unwrap_or("".to_string())
        )
    }
}

pub struct Object<S> {
    pub widget: Widget<S>,
    pub attributes: Vec<Attribute>,
    pub new_attributes: Vec<NewAttribute<S>>,
    pub children: Vec<Object<S>>,
}

/// Object: Actions
impl<S> Object<S> {
    pub fn new(widget: &str) -> Self {
        Object {
            widget: Widget::new(widget.into()),
            attributes: vec![],
            new_attributes: vec![],
            children: vec![],
        }
    }

    pub fn from_widget(widget: Widget<S>) -> Self {
        Object {
            widget,
            attributes: vec![],
            new_attributes: vec![],
            children: vec![],
        }
    }
}

impl<S> fmt::Debug for Object<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("widget", &self.widget)
            .field("attributes", &self.attributes)
            .field("new_attributes", &self.new_attributes)
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
    pub fn attr<V: ToString>(self, name: &str, value: V) -> Self {
        let name = match name {
            "class" => "className",
            _ => name,
        };

        self.attribute(Attribute(name.into(), value.to_string()))
    }

    fn attribute(mut self, attr: Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn new_attr(mut self, attr: NewAttribute<S>) -> Self {
        self.new_attributes.push(attr);
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
