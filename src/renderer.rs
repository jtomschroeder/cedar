
use std::collections::HashMap;

type Identifier = String;

// TODO: `enum` for 'kind'

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Create {
        id: Identifier,
        parent: Identifier,
        kind: String,
        value: Option<String>,
        attributes: HashMap<String, String>,
    },

    Update {
        id: Identifier,
        attribute: String,
        value: String,
    },

    Remove { id: Identifier },
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
