use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Key {
    pub pk: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Keys {
    pub keys: Vec<String>,
}
