use std::collections::HashMap;

use super::subrace::Subrace;
use serde::{Deserialize, Serialize};
use shoulda::Shoulda;

#[derive(Debug, Deserialize, Serialize, Shoulda, Clone)]
pub struct Race {
    pub name: String,
    pub flavor_text: Option<String>,
    pub features: HashMap<String, String>,
    pub subraces: Vec<Subrace>,
}
