use serde::{Deserialize, Serialize};

use super::subclass::Subclass;

#[derive(Debug, Deserialize, Serialize)]
pub struct Subclasses {
    pub name: String,
    pub features: Vec<u8>,
    pub entries: Vec<Subclass>,
}
