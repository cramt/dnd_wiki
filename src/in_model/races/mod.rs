pub mod race;
pub mod race_category;
pub mod subrace;

use serde::Deserialize;
use shoulda::Shoulda;

use self::{race::Race, race_category::RaceCategory};

#[derive(Debug, Deserialize, Shoulda)]
pub struct Races {
    pub race_categories: Vec<RaceCategory>,
    pub races: Vec<Race>,
}
