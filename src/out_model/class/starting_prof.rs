use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum StartingProfEntry {
    StrictSet(Vec<String>, u8),
    ChooseSet(u8, Vec<String>, u8),
    Empty,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartingProf {
    pub armor: StartingProfEntry,
    pub weapons: StartingProfEntry,
    pub tools: StartingProfEntry,
    pub saving_throws: StartingProfEntry,
    pub skills: StartingProfEntry,
}
