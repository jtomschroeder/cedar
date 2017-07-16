
#[derive(Clone, Debug)]
pub struct Node<T> {
    // pub kind: Kind,
    // pub attributes: Attributes,
    pub value: T,
    pub children: Vec<Node<T>>,
}

// impl<T> Node<T> {
//     pub fn is(&self, other: &Self) -> bool {
//         // self.kind == other.kind
//         false
//     }

//     pub fn eq(&self, other: &Self) -> bool {
//         // self.is(other) && self.attributes == other.attributes
//         false
//     }
// }

#[macro_export]
macro_rules! node {
    ($v:expr) => {
        $crate::Node {
            value: $v,
            children: vec![]
        }
    };

    ( $v:expr => $( $c:expr ),* ) => {{
        $crate::Node {
            value: $v,
            children: vec![ $( $c ),* ]
        }
    }};
}

pub type Path = Vec<Location>;

#[derive(Clone)]
pub struct Location {
    pub depth: usize,
    pub index: usize,
}

use std::fmt;

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Location({}:{})", self.depth, self.index)
    }
}

impl Location {
    fn new(depth: usize, index: usize) -> Self {
        Location { depth, index }
    }
}

#[derive(Debug)]
pub enum Operation<T> {
    Create(Node<T>),
    Delete,
    Update(T),
    Replace(Node<T>),
}

pub type Change<T> = (Path, Operation<T>);
pub type Changeset<T> = Vec<Change<T>>;

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

use Operation::*;

use std::collections::VecDeque;

type Nodes<T> = Vec<Node<T>>;

pub enum Difference {
    Kind,
    Value,
}

pub fn diff<T, F>(old: Nodes<T>, new: Nodes<T>, comparator: F) -> Changeset<T>
    where F: Fn(&Node<T>, &Node<T>) -> Option<Difference>
{
    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: DELETE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    UPDATE properties and check children

    // Breadth-First Traversal!

    let mut changeset = vec![];

    let mut queue = VecDeque::new();

    // TODO: is `level`/`depth` necessary? - implied by index of path?

    queue.push_back((old, new, 0, vec![]));

    while let Some((old, new, level, path)) = queue.pop_front() {
        for (n, pair) in zip(old, new).enumerate() {

            // Add current location to path
            let location = Location::new(level, n);
            let mut path = path.clone();
            path.push(location.clone());

            match pair {
                Pair::Left(_) => {
                    changeset.push((path.clone(), Delete));
                }

                Pair::Both(t, u) => {
                    // if t.type != u.type => replace u with t
                    // else if t != u (properties changes) => update and diff children
                    // else (if t == u) diff children

                    match comparator(&t, &u) {
                        Some(Difference::Kind) => {
                            changeset.push((path.clone(), Replace(u)));
                        }
                        cmp => {
                            if let Some(Difference::Value) = cmp {
                                changeset.push((path.clone(), Update(u.value)));
                            }

                            queue.push_back((t.children, u.children, level + 1, path));
                        }
                    }
                }

                Pair::Right(u) => {
                    changeset.push((path.clone(), Create(u)));
                }
            }
        }
    }

    changeset
}

fn patch() {}