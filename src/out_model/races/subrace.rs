use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shoulda::Shoulda;

#[derive(Debug, Serialize, Deserialize, Shoulda, Clone)]
pub struct Subrace {
    pub name: String,
    pub flavor_text: Option<String>,
    pub features: HashMap<String, String>,
}
