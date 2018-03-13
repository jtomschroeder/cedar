#![feature(proc_macro)]
#![feature(trace_macros)]

extern crate cedar;

use cedar::hypertext;

mod mouse {
    use cedar::{browser, Subscription};

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Position { x, y }
        }
    }

    pub struct Mouse<T> {
        f: fn(Position) -> T,
    }

    impl<T> Mouse<T> {
        pub fn moves(f: fn(Position) -> T) -> Self {
            Mouse { f }
        }
    }

    impl<T> Subscription for Mouse<T> {
        fn enable(&self) {
            browser::execute(
                r#"
                document.addEventListener('mousemove', (ev) => {
                    //console.log(ev);
                    window.post({ "Subscription": { "id": "123" } });
                })
                "#,
            );
        }

        fn disable(&self) {}
    }
}

use mouse::{Mouse, Position};

struct Model {
    position: Position,
    drag: Option<Drag>,
}

impl Model {
    fn position(&self) -> Position {
        match self.drag {
            None => self.position.clone(),
            Some(ref drag) => Position::new(
                self.position.x + drag.current.x - drag.start.x,
                self.position.y + drag.current.y - drag.start.y,
            ),
        }
    }
}

struct Drag {
    start: Position,
    current: Position,
}

#[derive(PartialEq)]
enum Message {
    DragStart(Position),
    DragAt(Position),
    DragEnd(Position),
}

fn update(model: Model, _message: &Message) -> Model {
    model
}

type Widget = cedar::dom::Object<Message>;

fn px<S: ToString>(i: S) -> String {
    format!("{}px", i.to_string())
}

fn style(model: &Model) -> String {
    let position = model.position();

    [
        ("background-color", "#3C8D2F"),
        ("cursor", "move"),
        ("width", "100px"),
        ("height", "100px"),
        ("border-radius", "4px"),
        ("position", "absolute"),
        ("left", &px(position.x)),
        ("top", &px(position.y)),
        ("color", "white"),
        ("display", "flex"),
        ("align-items", "center"),
        ("justify-content", "center"),
    ].iter()
        .map(|&(attr, val)| format!("{}: {}; ", attr, val))
        .fold(String::new(), |s, t| s + &t)
}

fn view(model: &Model) -> Widget {
    (hypertext! { |model|
        <div style={style(model)}>Drag Me!</div>
    })(model)
}

fn subscriptions(_: &Model) -> Mouse<Message> {
    Mouse::moves(Message::DragAt)

    // case model.drag of
    // Nothing -> Sub.none
    // Just _ -> Sub.batch [ Mouse.moves DragAt, Mouse.ups DragEnd ]
}

fn main() {
    cedar::programv((
        Model {
            position: Position::new(200, 200),
            drag: None,
        },
        update,
        view,
        subscriptions,
    ))
}
