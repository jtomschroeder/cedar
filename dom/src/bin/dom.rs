
#[macro_use]
extern crate dom;

#[derive(PartialEq, Debug)]
pub enum Kind {
    Stack,
    Button,
    Label,
}

#[derive(PartialEq, Debug)]
pub enum Attribute {
    Text(String),
}

pub type Attributes = Vec<Attribute>;

fn main() {
    objects();
}

use Kind::*;
use Attribute::*;

type Value = (Kind, Attributes);
type Node = dom::Node<Value>;

fn comparator(t: &Node, u: &Node) -> Option<dom::Difference> {
    if t.value.0 != u.value.0 {
        Some(dom::Difference::Kind)
    } else if t.value.1 != u.value.1 {
        Some(dom::Difference::Value)
    } else {
        None
    }
}

fn objects() {
    {
        let t = node![(Stack, vec![])];
        let u = node![(Stack, vec![])];

        let changeset = dom::diff(vec![t], vec![u], comparator);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![(Stack, vec![])];
        let u = node![(Button, vec![])];

        let changeset = dom::diff(vec![t], vec![u], comparator);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![(Label, vec![Text("".into())])];
        let u = node![(Label, vec![Text("!".into())])];

        let changeset = dom::diff(vec![t], vec![u], comparator);
        println!("changeset: {:?}", changeset);
    }

    {
        let u = node![(Stack, vec![]) 
                        => node![(Button, vec![])]
                         , node![(Label, vec![Text("!".into())])]
                         , node![(Button, vec![])]
                     ];

        let changeset = dom::diff(vec![], vec![u], comparator);
        println!("changeset: {:#?}", changeset);
    }
}