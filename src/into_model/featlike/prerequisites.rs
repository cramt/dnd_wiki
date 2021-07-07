use crate::in_model::featlike::prerequisites::PrerequisiteEntry as InEntry;
use crate::in_model::featlike::prerequisites::Prerequisites as In;
use crate::out_model::featlikes::prerequisites::PrerequisiteEntry as OutEntry;
use crate::out_model::featlikes::prerequisites::Prerequisites as Out;

impl From<InEntry> for OutEntry {
    fn from(x: InEntry) -> Self {
        match x {
            InEntry::Leaf(x) => Self::Leaf(x),
            InEntry::Or(x) => Self::Or(x.into_iter().map(|x| x.into()).collect()),
        }
    }
}

impl From<In> for Out {
    fn from(x: In) -> Self {
        Self(x.0.into_iter().map(|x| x.into()).collect())
    }
}
