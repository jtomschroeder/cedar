
# cedar :evergreen_tree:

`cedar` is a functional-reactive framework.

[![crates.io](https://img.shields.io/crates/v/cedar.svg)](https://crates.io/crates/cedar)
![License](https://img.shields.io/crates/l/cedar.svg)
[![Build Status](https://travis-ci.org/jtomschroeder/cedar.svg?branch=master)](https://travis-ci.org/jtomschroeder/cedar)

**Status:** `cedar` is in the *alpha* stage - not yet ready for prime-time.

### Usage

Add `cedar` to your project via *cargo*.

#### Creating buttons & *reactive* text :rocket:

```rust
extern crate cedar;

use cedar::dom;
use cedar::dom::Builder;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: Message) -> Model {
    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> dom::Object<Message> {
    dom::stack()
        .add(dom::button().text("+".into()).click(Message::Increment))
        .add(dom::label().text(model.to_string()))
        .add(dom::button().text("-".into()).click(Message::Decrement))
}

fn main() {
    cedar::program(0, update, view)
}
```

### Design

A `cedar` application is composed of a *model*, *update*, and *view*. **TODO: expand on this....**

### Credits

Inspired by:
- [elm-lang](http://elm-lang.org)
- [shoes-rb](http://shoesrb.com)

`cedar` is Copyright © Tom Schroeder <j.tom.schroeder@gmail.com> and released under MIT license.
