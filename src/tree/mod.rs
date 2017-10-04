
mod path;
mod zipper;

use std::collections::VecDeque;

use self::zipper::{zip, Pair};
pub use self::path::Path;

pub trait Vertex {
    fn children(&self) -> &[Self]
    where
        Self: Sized;

    fn compare(&self, other: &Self) -> Option<Difference>;

    fn find(&self, path: &Path) -> Option<&Self>
    where
        Self: Sized,
    {
        let path = path.raw();

        let mut queue = VecDeque::new();
        queue.push_back((path, 0, self));

        while let Some((path, i, node)) = queue.pop_front() {
            match path.len() {
                0 => {}

                1 if i == path[0] => return Some(node),

                _ if i == path[0] => {
                    for (n, child) in node.children().iter().enumerate() {
                        queue.push_back((&path[1..], n, child));
                    }
                }

                _ => {}
            }
        }

        None
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

pub fn diff<V>(old: &V, new: &V) -> Changeset
where
    V: Vertex,
{
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
