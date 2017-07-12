
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

// TODO: turn `level` into `path`

fn diff<T>(old: &[Box<Node<T>>], new: &[Box<Node<T>>], level: usize)
    where T: fmt::Debug + PartialEq
{
    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: REMOVE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    update properties and keep going

    // let display = |t: &[Box<Node<T>>]| {
    //     t.iter()
    //         .map(|n| format!("{:?}", n.value))
    //         .collect::<Vec<_>>()
    // };
    // println!("{:?} :: {:?} :: {:?}", display(old), display(new), level);

    for pair in zip(old, new) {
        match pair {
            Pair::Left(t) => {
                println!("Delete {:?} @ {}", t.value, level);
            }

            Pair::Both(t, u) => {
                // if t.type != u.type => replace u with t
                // else if t != u (properties changes) => update and diff children
                // else (if t == u) diff children

                // TODO: compare types (don't diff children if 'replace')

                if t.value != u.value {
                    println!("Update {:?} with {:?} @ {}", t.value, u.value, level);
                }

                diff(&t.children, &u.children, level + 1)
            }

            Pair::Right(u) => {
                println!("Create {:?} @ {}", u.value, level);
            }
        }
    }
}

fn patch() {}

fn main() {
    let t = node![ 
        0 => node![1], 
             node![2 => node![3] 
        ] 
    ];

    let u = node![1];

    diff(&[Box::new(t)], &[Box::new(u)], 0);
}
