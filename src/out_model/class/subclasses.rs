use serde::{Deserialize, Serialize};

use super::subclass::Subclass;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subclasses {
    pub name: String,
    pub features: Vec<u8>,
    pub entries: Vec<Subclass>,
}
