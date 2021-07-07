
use serde::{Deserialize, Serialize};

use super::prerequisites::Prerequisites;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Featlike {
    pub name: String,
    pub prerequisites: Prerequisites,
    pub flavor_text: String,
    pub body: String,
}