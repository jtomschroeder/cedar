
mod zipper;

use std::fmt;
use std::collections::VecDeque;

use self::zipper::{zip, Pair};

pub trait Vertex {
    fn children(&self) -> &[Self]
    where
        Self: Sized;

    fn compare(&self, other: &Self) -> Option<Difference>;
}

#[derive(Clone, Debug)]
pub struct Path {
    path: Vec<usize>,
}

impl Path {
    pub fn new() -> Self {
        Path { path: vec![0] }
    }

    pub fn from_vec(path: Vec<usize>) -> Self {
        Path { path }
    }

    pub fn push(&mut self, element: usize) {
        self.path.push(element)
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn raw(&self) -> &[usize] {
        &self.path
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create string representation of path (e.g. 0.0.1.3)

        if self.path.is_empty() {
            write!(f, "")
        } else if self.path.len() == 1 {
            write!(f, "{}", self.path[0])
        } else {
            let id = (&self.path[1..]).iter().fold(
                self.path[0].to_string(),
                |id, n| id + &format!(".{}", n),
            );
            write!(f, "{}", id)
        }
    }
}


#[derive(Debug)]
pub enum Operation {
    Create,
    Delete,
    Update,
    Replace,
}

pub type Change = (Path, Operation);
pub type Changeset = Vec<Change>;

pub enum Difference {
    Kind,
    Value,
}

pub fn diff<V: Vertex>(old: &V, new: &V) -> Changeset {
    use self::Operation::*;

    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: DELETE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    UPDATE properties and check children

    // Breadth-First Traversal!

    let mut changeset = vec![];

    let path = Path::new();
    let mut queue = VecDeque::new();

    // TODO: this code is same as below... (DRY)
    match old.compare(&new) {
        Some(Difference::Kind) => changeset.push((path, Replace)),
        cmp => {
            if let Some(Difference::Value) = cmp {
                changeset.push((path.clone(), Update));
            }

            queue.push_back((old.children(), new.children(), path));
        }
    }

    while let Some((old, new, path)) = queue.pop_front() {
        for (n, pair) in zip(old, new).enumerate() {

            // Add current location to path
            let mut path = path.clone();
            path.push(n);

            match pair {
                Pair::Left(_) => changeset.push((path, Delete)),
                Pair::Right(_) => changeset.push((path, Create)),

                Pair::Both(t, u) => {
                    //       if t.type != u.type            => replace u with t
                    // else  if t != u (properties changes) => update and diff children
                    // else (if t == u)                     => diff children

                    match t.compare(&u) {
                        Some(Difference::Kind) => changeset.push((path, Replace)),
                        cmp => {
                            if let Some(Difference::Value) = cmp {
                                changeset.push((path.clone(), Update));
                            }

                            queue.push_back((t.children(), u.children(), path));
                        }
                    }
                }
            }
        }
    }

    changeset
}
