use std::collections::HashMap;
use json;

type Identifier = String;

// TODO: `enum` for kind?

#[derive(Serialize, Deserialize, Debug)]
pub enum Update {
    Text(String),
    Attributes(HashMap<String, String>),
}

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
        value: Update,
    },

    Remove {
        id: Identifier,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Click { id: Identifier },
    Input { id: Identifier, value: String },
    Keydown { id: Identifier, code: u32 },
}
