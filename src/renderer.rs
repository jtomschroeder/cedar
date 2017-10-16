
use std::collections::HashMap;

type Identifier = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Create {
        id: Identifier,
        kind: String,
        attributes: HashMap<String, String>,
    },

    Update(Identifier, String, String), // ID * Attribute * Value

    Remove(Identifier), // ID
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Click { id: Identifier },
    Change { id: Identifier, value: String },
}

pub trait Renderer {
    fn send(&self, Command);
    fn recv(&self) -> Event;
}
