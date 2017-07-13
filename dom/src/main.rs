
// extern crate typed_arena;
// extern crate arena_tree;

use std::fmt;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
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

#[derive(PartialEq, Debug)]
struct Path {
    pub depth: usize,
    pub index: usize,
}

impl Path {
    fn new(depth: usize, index: usize) -> Self {
        Path { depth, index }
    }
}

#[derive(PartialEq, Debug)]
enum Operation<T> {
    Create(T),
    Delete,
    Update(T, T),
    Replace(T, T),
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

// TODO: drain old & new trees instead slicing and cloning
// TODO: add param to diff `FnMut(Pair<T, T>) -> Operation` to decouple determining operations from `diff`?

trait Diffable {
    fn kind(&self, &Self) -> bool;
    fn value(&self, &Self) -> bool;
}

impl Diffable for u32 {
    fn kind(&self, _: &u32) -> bool {
        true
    }

    fn value(&self, other: &u32) -> bool {
        self == other
    }
}

fn diff<T>(old: &[Node<T>], new: &[Node<T>], level: usize) -> Changeset<T>
    where T: fmt::Debug + Clone + Diffable
{
    // -      if `old` doesn't exist: CREATE new
    // - else if `new` doesn't exist: REMOVE old
    // - else if old.type != new.type: REPLACE old with new
    // - else    update properties and keep going

    let mut changeset = vec![];

    for (n, pair) in zip(old, new).enumerate() {
        match pair {
            Pair::Left(t) => {
                // println!("Delete {:?} @ {}:{}", t.value, level, n);
                changeset.push((Path::new(level, n), Delete));
            }

            Pair::Both(t, u) => {
                // if t.type != u.type => replace u with t
                // else if t != u (properties changes) => update and diff children
                // else (if t == u) diff children

                if !t.value.kind(&u.value) {
                    // println!("Replace {:?} with {:?} @ {}:{}", t.value, u.value, level, n);
                    changeset.push((Path::new(level, n),
                                    Replace(t.value.clone(), u.value.clone())));
                } else {
                    if !t.value.value(&u.value) {
                        // println!("Update {:?} with {:?} @ {}:{}", t.value, u.value, level, n);
                        changeset.push((Path::new(level, n),
                                        Update(t.value.clone(), u.value.clone())));
                    }

                    changeset.extend(diff(&t.children, &u.children, level + 1));
                }
            }

            Pair::Right(u) => {
                // println!("Create {:?} @ {}:{}", u.value, level, n);
                changeset.push((Path::new(level, n), Create(u.value.clone())));
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

        let changeset = diff(&[t], &[u], 0);
        // println!("changeset: {:#?}", changeset);

        assert_eq!(changeset,
                   vec![(Path::new(0, 0), Update(0, 1)),
                        (Path::new(1, 0), Delete),
                        (Path::new(1, 1), Delete)]);
    }

    {
        let t = node![0];
        let u = node![1];

        let changeset = diff(&[t], &[u], 0);
        // println!("changeset: {:#?}", changeset);

        assert_eq!(changeset, vec![(Path::new(0, 0), Update(0, 1))]);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Div,
    Button,
    Text(String),
}

impl Diffable for Object {
    fn kind(&self, other: &Object) -> bool {
        use Object::*;
        match (self, other) {
            (&Div, &Div) |
            (&Button, &Button) |
            (&Text(_), &Text(_)) => true,

            _ => false,
        }
    }

    fn value(&self, other: &Object) -> bool {
        self == other
    }
}

fn objects() {
    use Object::*;

    {
        let t = node![Div];
        let u = node![Div];

        let changeset = diff(&[t], &[u], 0);
        println!("changeset: {:#?}", changeset);
    }

    {
        let t = node![Div];
        let u = node![Button];

        let changeset = diff(&[t], &[u], 0);
        println!("changeset: {:#?}", changeset);
    }

    {
        let t = node![Text("".into())];
        let u = node![Text("!".into())];

        let changeset = diff(&[t], &[u], 0);
        println!("changeset: {:#?}", changeset);
    }
}