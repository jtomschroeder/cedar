
extern crate typed_arena;
extern crate arena_tree;

use std::fmt;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Box<Node<T>>>,
}

macro_rules! node {
    ($v:expr) => {
        Node {
            value: $v,
            children: vec![]
        }
    };

    ( $v:expr => $( $c:expr ),* ) => {{
        let mut children = vec![];

        $( children.push(Box::new($c)); )*

        Node { value: $v, children }
    }};
}

// Path :: XPath-like location into tree

// enum Operation = Add(Tree) | Remove | Update(Tree)

// type Changeset = Vec<(Path, Operation)>

// diff :: Tree -> Tree -> Changeset
// - can add Path to the parameters to create a recursive implementation
//
// `diff(old, new, Path::Root)`
//

// Iterator Zip with both
// enum {
//     Left(T), Both(T, U), Right(U)
// }

fn diff<T>(t: &[Box<Node<T>>], u: &[Box<Node<T>>], level: usize)
    where T: fmt::Debug
{
    println!("{:?} :: {:?} :: {:?}", t, u, level);

    // for ()
}

fn patch() {}

fn main() {
    // println!("Hello, world!");

    // let arena = typed_arena::Arena::new();

    // let a = arena.alloc(arena_tree::Node::new(Object::Div));
    // let b = arena.alloc(arena_tree::Node::new(Object::Div));

    // a.append(b);

    // for node in a.descendants() {
    //     println!("{:?}", node.data);
    // }

    let t = node![ 0 => node![1], node![2] ];
    let u = node![1];

    // println!("{:#?}", (t, u));

    diff(&[Box::new(t)], &[Box::new(u)], 0);
}
