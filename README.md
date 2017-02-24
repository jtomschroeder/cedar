
# cedar :evergreen_tree:

`cedar` is a functional-reactive GUI library.

[![crates.io](https://img.shields.io/crates/v/cedar.svg)](https://crates.io/crates/cedar)
![License](https://img.shields.io/crates/l/cedar.svg)

**Status:** `cedar` is in the *alpha* stage and not yet ready for prime-time.

### Documentation

[`cedar` documentation here!](https://docs.rs/cedar)

### Usage

Add this to `Cargo.toml`:

```toml
[dependencies]
cedar = { git = "https://github.com/jtomschroeder/cedar" }
```

and this to the root of your crate:

```rust
extern crate cedar;
```

#### Creating buttons & *reactive* text :rocket:

```rust
extern crate cedar;

type Model = i32;

enum Message {
    Increment,
    Decrement,
}

fn update(model: &Model, message: Message) -> Model {
    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

fn view() -> cedar::View<Model, Message> {
    cedar::View::new()
        .button(|button| {
            button.text("+")
                .click(|| Message::Increment)
        })
        .label(|label| label.text(Model::to_string))
        .button(|button| {
            button.text("-")
                .click(|| Message::Decrement)
        })
}

fn main() {
    cedar::Application::new(0, update, view).run()
}
```

### Design

A `cedar` application is composed of a *model*, *update*, and *view*.

### Credits

Inspired by:
- [elm-lang](http://elm-lang.org)
- [shoes-rb](http://shoesrb.com)

`cedar` is Copyright © Tom Schroeder <j.tom.schroeder@gmail.com> and released under MIT license.
