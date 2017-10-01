
use std::collections::VecDeque;

pub trait Vertex {
    fn children(&self) -> &[Self]
    where
        Self: Sized;

    fn compare(&self, other: &Self) -> Option<Difference>;
}

pub type Path = Vec<usize>;

#[derive(Debug)]
pub enum Operation {
    Create,
    Delete,
    Update,
    Replace,
}

pub type Change = (Path, Operation);
pub type Changeset = Vec<Change>;

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
where
    I: Iterator,
    J: Iterator,
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
where
    I: IntoIterator,
    J: IntoIterator,
{
    Zip {
        i: i.into_iter(),
        j: j.into_iter(),
    }
}

pub enum Difference {
    Kind,
    Value,
}

pub fn diff<V: Vertex>(old: V, new: V) -> Changeset {
    use self::Operation::*;

    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: DELETE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    UPDATE properties and check children

    // Breadth-First Traversal!

    let mut changeset = vec![];

    let old: &[V] = &[old];
    let new: &[V] = &[new];

    let mut queue = VecDeque::new();
    queue.push_back((old, new, vec![]));

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
