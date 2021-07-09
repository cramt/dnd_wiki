pub mod race;
pub mod subrace;

use self::race::Race;
use serde::{Deserialize, Serialize};
use shoulda::Shoulda;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Shoulda, Clone)]
pub struct Races(pub HashMap<String, (Option<String>, Vec<Race>)>);
