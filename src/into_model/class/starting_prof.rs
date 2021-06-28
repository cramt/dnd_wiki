use crate::model::class::starting_prof::StartingProf as In;
use crate::model::class::starting_prof::StartingProfEntry as InEntry;
use crate::out_model::class::starting_prof::StartingProf as Out;
use crate::out_model::class::starting_prof::StartingProfEntry as OutEntry;

impl Into<InEntry> for OutEntry {
    fn into(self) -> InEntry {
        match self {
            OutEntry::StrictSet(a, b) => InEntry::StrictSet(a, b),
            OutEntry::ChooseSet(a, b, c) => InEntry::ChooseSet(a, b, c),
            OutEntry::Empty => InEntry::Empty,
        }
    }
}

impl Into<OutEntry> for InEntry {
    fn into(self) -> OutEntry {
        match self {
            InEntry::StrictSet(a, b) => OutEntry::StrictSet(a, b),
            InEntry::ChooseSet(a, b, c) => OutEntry::ChooseSet(a, b, c),
            InEntry::Empty => OutEntry::Empty,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        let Self {
            armor,
            weapons,
            tools,
            saving_throws,
            skills,
        } = self;
        let armor = armor.into();
        let weapons = weapons.into();
        let tools = tools.into();
        let saving_throws = saving_throws.into();
        let skills = skills.into();
        In {
            armor,
            weapons,
            tools,
            saving_throws,
            skills,
        }
    }
}

impl Into<Out> for In {
    fn into(self) -> Out {
        let Self {
            armor,
            weapons,
            tools,
            saving_throws,
            skills,
        } = self;
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
