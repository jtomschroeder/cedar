
use tree;

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

pub type Attributes<S> = Vec<Attribute<S>>;

pub type Value<S> = (Kind, Attributes<S>);
pub type Node<S> = tree::Node<Value<S>>;

pub type Change<S> = tree::Change<Value<S>>;
pub type Changeset<S> = tree::Changeset<Value<S>>;

pub fn diff<S>(old: Node<S>, new: Node<S>) -> Changeset<S>
    where S: PartialEq
{
    fn comparator<S>(t: &Node<S>, u: &Node<S>) -> Option<tree::Difference>
        where S: PartialEq
    {
        if t.value.0 != u.value.0 {
            Some(tree::Difference::Kind)
        } else if t.value.1 != u.value.1 {
            Some(tree::Difference::Value)
        } else {
            None
        }

    }

    tree::diff(vec![old], vec![new], comparator)
}
