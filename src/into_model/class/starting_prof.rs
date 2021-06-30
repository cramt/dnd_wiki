use crate::in_model::class::starting_prof::StartingProf as In;
use crate::in_model::class::starting_prof::StartingProfEntry as InEntry;
use crate::out_model::class::starting_prof::StartingProf as Out;
use crate::out_model::class::starting_prof::StartingProfEntry as OutEntry;

impl From<InEntry> for OutEntry {
    fn from(val: InEntry) -> Self {
        match val {
            InEntry::StrictSet(a, b) => OutEntry::StrictSet(a, b),
            InEntry::ChooseSet(a, b, c) => OutEntry::ChooseSet(a, b, c),
            InEntry::Empty => OutEntry::Empty,
        }
    }
}

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            armor,
            weapons,
            tools,
            saving_throws,
            skills,
        } = val;
        let armor = armor.into();
        let weapons = weapons.into();
        let tools = tools.into();
        let saving_throws = saving_throws.into();
        let skills = skills.into();
        Out {
            armor,
            weapons,
            tools,
            saving_throws,
            skills,
        }
    }
}
