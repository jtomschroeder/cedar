
#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

enum Kind {}

enum Attribute {}

struct DomNode {
    kind: Kind,
    attributes: Vec<Attribute>,
    children: Vec<DomNode>,
}

macro_rules! node {
    ($v:expr) => {
        Node {
            value: $v,
            children: vec![]
        }
    };

    ( $v:expr => $( $c:expr ),* ) => {{
        Node { 
            value: $v, 
            children: vec![ $( $c ),* ]
        }
    }};
}

type Path = Vec<Location>;

#[derive(PartialEq, Clone)]
struct Location {
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

#[derive(PartialEq, Debug)]
enum Operation<T> {
    Create(Node<T>),
    Delete,
    Update(T, T),
    Replace(Node<T>),
}

type Changeset<T> = Vec<(Path, Operation<T>)>;

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

// TODO: add param to diff `FnMut(Pair<T, T>) -> Operation` to decouple determining operations from `diff`?

trait Diffable {
    fn is(&self, &Self) -> bool;
    fn eq(&self, &Self) -> bool;
}

impl Diffable for u32 {
    fn is(&self, _: &u32) -> bool {
        true
    }

    fn eq(&self, other: &u32) -> bool {
        self == other
    }
}

use std::collections::VecDeque;

type Nodes<T> = Vec<Node<T>>;

fn diff<T>(old: Nodes<T>, new: Nodes<T>) -> Changeset<T>
    where T: Diffable
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

                    if !t.value.is(&u.value) {
                        changeset.push((path.clone(), Replace(u)));
                    } else {
                        if !t.value.eq(&u.value) {
                            changeset.push((path.clone(), Update(t.value, u.value)));
                        }

                        queue.push_back((t.children, u.children, level + 1, path));
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

fn main() {
    integers();
    objects();
}

fn integers() {
    {
        let t = node![0 => node![1], node![2 => node![3]]];
        let u = node![1];

        let changeset = diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);

        // assert_eq!(changeset,
        //            vec![(Location::new(0, 0), Update(0, 1)),
        //                 (Location::new(1, 0), Delete),
        //                 (Location::new(1, 1), Delete)]);
    }

    {
        let t = node![0];
        let u = node![1];

        let changeset = diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);

        // assert_eq!(changeset, vec![(Location::new(0, 0), Update(0, 1))]);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Stack,
    Button,
    Text(String),
}

impl Diffable for Object {
    fn is(&self, other: &Object) -> bool {
        use Object::*;
        match (self, other) {
            (&Stack, &Stack) |
            (&Button, &Button) |
            (&Text(_), &Text(_)) => true,

            _ => false,
        }
    }

    fn eq(&self, other: &Object) -> bool {
        self == other
    }
}

fn objects() {
    use Object::*;

    {
        let t = node![Stack];
        let u = node![Stack];

        let changeset = diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![Stack];
        let u = node![Button];

        let changeset = diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![Text("".into())];
        let u = node![Text("!".into())];

        let changeset = diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }
}