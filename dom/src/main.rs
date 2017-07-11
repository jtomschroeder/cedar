
extern crate typed_arena;
extern crate arena_tree;

#[derive(Clone, Debug)]
enum Object {
    Div,
    Button,
    Text(String),
}

// Path :: XPath-like location into tree

// enum Operation = Add(Tree) | Remove | Update(Tree)

// type Changeset = Vec<(Path, Operation)>

// diff :: Tree -> Tree -> Changeset
// - can add Path to the parameters to create a recursive implementation
//
// `diff(old, new, Path::Root)`
//

fn diff() {}

fn patch() {}

fn main() {
    println!("Hello, world!");

    let arena = typed_arena::Arena::new();

    let a = arena.alloc(arena_tree::Node::new(Object::Div));
    let b = arena.alloc(arena_tree::Node::new(Object::Div));

    a.append(b);

    for node in a.descendants() {
        println!("{:?}", node.data);
    }
}
