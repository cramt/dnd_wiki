use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum StartingProfEntry {
    StrictSet(Vec<String>, u8),
    ChooseSet(u8, Vec<String>, u8),
    Empty,
}

impl Default for StartingProfEntry {
    fn default() -> Self {
        StartingProfEntry::Empty
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartingProf {
    armor: StartingProfEntry,
    weapons: StartingProfEntry,
    tools: StartingProfEntry,
    saving_throws: StartingProfEntry,
    skills: StartingProfEntry,
}
