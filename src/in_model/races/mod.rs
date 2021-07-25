pub mod race;
pub mod race_category;
pub mod subrace;

use serde::Deserialize;
use shoulda::Shoulda;
use crate::in_model::vec_of_path_or_struct::vec_of_path_or_struct;

use self::{race::Race, race_category::RaceCategory};

#[derive(Debug, Deserialize, Shoulda)]
pub struct Races {
    pub race_categories: Vec<RaceCategory>,
    #[serde(deserialize_with = "vec_of_path_or_struct")]
    pub races: Vec<Race>,
}
