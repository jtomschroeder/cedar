
// extern crate typed_arena;
// extern crate arena_tree;

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

enum Pair<T, U> {
    Left(T),
    Both(T, U),
    Right(U),
}

struct Zip<I, J> {
    i: I,
    j: J,
}

impl<I, J> Iterator for Zip<I, J>
    where I: Iterator,
          J: Iterator
{
    type Item = Pair<I::Item, J::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.i.next(), self.j.next()) {
            (Some(i), Some(j)) => Some(Pair::Both(i, j)),
            (Some(i), _) => Some(Pair::Left(i)),
            (_, Some(j)) => Some(Pair::Right(j)),
            _ => None,
        }
    }
}

fn zip<I, J>(i: I, j: J) -> Zip<I::IntoIter, J::IntoIter>
    where I: IntoIterator,
          J: IntoIterator
{
    Zip {
        i: i.into_iter(),
        j: j.into_iter(),
    }
}

fn diff<T>(t: &[Box<Node<T>>], u: &[Box<Node<T>>], level: usize)
    where T: fmt::Debug
{
    let display = |t: &[Box<Node<T>>]| {
        t.iter()
            .map(|n| format!("{:?}", n.value))
            .collect::<Vec<_>>()
    };

    println!("{:?} :: {:?} :: {:?}", display(t), display(u), level);

    // for (t, u) in t.iter().zip(u) {
    //     diff(&t.children, &u.children, level + 1);
    // }

    for pair in zip(t, u) {
        match pair {
            Pair::Left(t) => diff(&t.children, &[], level + 1),
            Pair::Both(t, u) => diff(&t.children, &u.children, level + 1),
            Pair::Right(u) => diff(&[], &u.children, level + 1),
        }
    }
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

    let t = node![ 0 => node![1], node![2 => node![3] ] ];
    let u = node![1];

    // println!("{:#?}", (t, u));

    diff(&[Box::new(t)], &[Box::new(u)], 0);
}
