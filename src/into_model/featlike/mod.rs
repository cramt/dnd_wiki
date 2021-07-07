pub mod prerequisites;

use crate::in_model::featlike::Featlike as In;
use crate::out_model::featlikes::featlike::Featlike as Out;
use crate::out_model::featlikes::Featlikes as Outs;

impl From<In> for Out {
    fn from(x: In) -> Self {
        let In {
            name,
            prerequisites,
            flavor_text,
            body,
        } = x;
        let prerequisites = prerequisites.into();
        Self {
            name,
            prerequisites,
            flavor_text,
            body,
        }
    }
}

impl From<(Vec<In>, String)> for Outs {
    fn from(x: (Vec<In>, String)) -> Self {
        let (entries, name) = x;
        let entries = entries.into_iter().map(|x| x.into()).collect();
        Self { name, entries }
    }
}
