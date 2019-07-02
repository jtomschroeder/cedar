mod path;
mod zipper;

use std::collections::VecDeque;

use self::zipper::{zip, Pair};
pub use self::path::Path;

pub trait Vertex {
    fn children(&self) -> &[Self]
    where
        Self: Sized;

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

                _ if i == path[0] => for (n, child) in node.children().iter().enumerate() {
                    queue.push_back((&path[1..], n, child));
                },

                _ => {}
            }
        }

        None
    }

    fn traverse<D>(&self, root: &Path, mut delegate: D)
    where
        Self: Sized,
        D: FnMut(&Path, &Self),
    {
        let path = root.clone();

        let mut queue = VecDeque::new();
        queue.push_back((path, self));

        while let Some((path, node)) = queue.pop_front() {
            delegate(&path, node);

            for (n, child) in node.children().iter().enumerate() {
                let mut path = path.clone();
                path.push(n);

                queue.push_back((path, child));
            }
        }
    }
}

pub trait Comparable {
    fn compare(&self, other: &Self) -> Option<Difference>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    V: Vertex + Comparable,
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

// TODO: REALLY need to build out these tests!

#[cfg(test)]
mod tests {
    use crate::tree;

    #[derive(PartialEq, Debug)]
    enum Kind {
        This,
        That,
    }

    #[derive(Debug)]
    struct Object {
        kind: Kind,
        value: u32,

        children: Vec<Object>,
    }

    fn this(value: u32, children: Vec<Object>) -> Object {
        Object {
            kind: Kind::This,
            value,
            children,
        }
    }

    fn that(value: u32, children: Vec<Object>) -> Object {
        Object {
            kind: Kind::That,
            value,
            children,
        }
    }

    impl tree::Vertex for Object {
        fn children(&self) -> &[Self] {
            &self.children
        }
    }

    impl tree::Comparable for Object {
        fn compare(&self, other: &Self) -> Option<tree::Difference> {
            if self.kind != other.kind {
                Some(tree::Difference::Kind)
            } else if self.value != other.value {
                Some(tree::Difference::Value)
            } else {
                None
            }
        }
    }

    #[test]
    fn same_tree() {
        let tree = that(0, vec![this(1, vec![]), this(2, vec![])]);

        let changeset = tree::diff(&tree, &tree);
        assert!(changeset.is_empty());
    }

    #[test]
    fn tree() {
        let left = that(0, vec![this(1, vec![]), this(2, vec![])]);
        let right = that(0, vec![this(2, vec![])]);

        let changeset = tree::diff(&left, &right);
        println!("changeset: {:?}", changeset);
    }
}
