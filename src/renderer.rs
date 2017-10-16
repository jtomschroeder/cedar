
use std::collections::HashMap;

type Identifier = String;
type Frame = (f32, f32, f32, f32); // (x, y, w, h)

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Create {
        id: Identifier,
        kind: String,
        frame: Frame,
        attributes: HashMap<String, String>,
    },

    Update(Identifier, String, String), // ID -> Attribute

    Move(Vec<(Identifier, Frame)>),

    Remove(Identifier), // ID
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Click { id: Identifier },
    Change { id: Identifier, value: String },

    Resize { width: f32, height: f32 },
}

pub trait Renderer {
    fn send(&self, Command);
    fn recv(&self) -> Event;
}
