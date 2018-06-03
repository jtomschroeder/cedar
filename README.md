
# cedar :evergreen_tree:

`cedar` is a Rust framework for building visual/interactive applications.

[![crates.io](https://img.shields.io/crates/v/cedar.svg)](https://crates.io/crates/cedar)
![License](https://img.shields.io/crates/l/cedar.svg)
[![Build Status](https://travis-ci.org/jtomschroeder/cedar.svg?branch=master)](https://travis-ci.org/jtomschroeder/cedar)

**Note:** `cedar` is in the *experimental* stage and rapidly evolving.

### Getting started!

Add the following dependency to your `Cargo.toml`:

```toml
cedar = { git = "https://github.com/jtomschroeder/cedar" }
```

### Example: creating buttons & *reactive* text :rocket:

```rust
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate cedar;

use cedar::hypertext;

type Model = i32;

#[derive(PartialEq)]
enum Message { Increment, Decrement }

fn update(model: Model, message: &Message) -> Model {
    match message {
        &Message::Increment => model + 1,
        &Message::Decrement => model - 1,
    }
}

fn view(model: &Model) -> cedar::dom::Object<Message> {
    (hypertext! { |model|
        <div>
            <button click={Message::Increment}> + </button>
            <div>{model}</div>
            <button click={Message::Decrement}> - </button>
        </div>
    })(model)
}

fn main() { cedar::app(0, update, view) }
```

### Design

A `cedar` application is composed of a *model*, *update*, and *view* - all declared up-front:

- *model*: the state of our app
- *update*: how to update our state based on a message (i.e. event)
- *view*: how to transform our state into UI 'widgets'

This architecture is powerful, yet simple. Using a declarative approach, we can achieve impressive reactivity without having to worry about threads, locks, event routing, or view controllers.

Check out the examples!

### Credits

Inspired by:
- [elm-lang](http://elm-lang.org)
- [shoes-rb](http://shoesrb.com)

`cedar` is released under the MIT license.
