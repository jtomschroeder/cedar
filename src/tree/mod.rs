
#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

#[macro_export]
macro_rules! node {
    ($v:expr) => {
        $crate::tree::Node {
            value: $v,
            children: vec![]
        }
    };

    ( $v:expr => $( $c:expr ),* ) => {{
        $crate::tree::Node {
            value: $v,
            children: vec![ $( $c ),* ]
        }
    }};
}

pub type Path = Vec<Location>;

#[derive(PartialEq, Clone)]
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
    pub fn new(depth: usize, index: usize) -> Self {
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

use std::collections::VecDeque;

type Nodes<T> = Vec<Node<T>>;

pub enum Difference {
    Kind,
    Value,
}

pub fn diff<T, F>(old: Nodes<T>, new: Nodes<T>, comparator: F) -> Changeset<T>
where
    F: Fn(&Node<T>, &Node<T>) -> Option<Difference>,
{
    use self::Operation::*;

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

#[cfg(test)]
mod test {
    use tree;

    #[derive(PartialEq, Debug)]
    enum Kind {
        Stack,
        Button,
        Label,
    }

    #[derive(PartialEq, Debug)]
    enum Attribute {
        Text(String),
    }

    type Attributes = Vec<Attribute>;

    type Value = (Kind, Attributes);
    type Node = tree::Node<Value>;

    fn comparator(t: &Node, u: &Node) -> Option<tree::Difference> {
        if t.value.0 != u.value.0 {
            Some(tree::Difference::Kind)
        } else if t.value.1 != u.value.1 {
            Some(tree::Difference::Value)
        } else {
            None
        }
    }

    #[test]
    fn objects() {
        use self::Kind::*;
        use self::Attribute::*;

        use tree::Location;
        use tree::Operation::*;

        {
            let t = node![(Stack, vec![])];
            let u = node![(Stack, vec![])];

            let changeset = tree::diff(vec![t], vec![u], comparator);
            assert!(changeset.is_empty());
        }

        {
            let t = node![(Stack, vec![])];
            let u = node![(Button, vec![])];

            let mut changeset = tree::diff(vec![t], vec![u], comparator);
            assert_eq!(changeset.len(), 1);

            let (location, operation) = changeset.remove(0);
            assert_eq!(&location, &[Location::new(0, 0)]);

            match operation {
                Replace(node) => {
                    let (kind, _) = node.value;
                    assert_eq!(kind, Button);
                }
                _ => panic!("Wrong operation!"),
            }
        }

        {
            let t = node![(Label, vec![Text("".into())])];
            let u = node![(Label, vec![Text("!".into())])];

            let mut changeset = tree::diff(vec![t], vec![u], comparator);
            assert_eq!(changeset.len(), 1);

            let (location, operation) = changeset.remove(0);
            assert_eq!(&location, &[Location::new(0, 0)]);

            match operation {
                Update((kind, attrs)) => {
                    assert_eq!(kind, Label);
                    assert_eq!(&attrs, &[Text("!".into())]);
                }
                _ => panic!("Wrong operation!"),
            }
        }

        {
            let u =
                node![(Stack, vec![]) 
                        => node![(Button, vec![])]
                         , node![(Label, vec![Text("!".into())])]
                         , node![(Button, vec![])]
                     ];

            let mut changeset = tree::diff(vec![], vec![u], comparator);
            assert_eq!(changeset.len(), 1);

            let (location, operation) = changeset.remove(0);
            assert_eq!(&location, &[Location::new(0, 0)]);

            match operation {
                Create(..) => {}
                _ => panic!("Wrong operation!"),
            }
        }
    }
}
