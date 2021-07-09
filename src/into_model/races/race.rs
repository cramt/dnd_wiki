use crate::in_model::races::race::Race as In;
use crate::out_model::races::race::Race as Out;

impl From<In> for Out {
    fn from(x: In) -> Self {
        let In {
            name,
            category: _,
            features,
            flavor_text,
            subraces,
        } = x;
        let subraces = subraces.into_iter().map(|x| x.into()).collect();
        Self {
            name,
            flavor_text,
            features,
            subraces,
        }
    }
}
