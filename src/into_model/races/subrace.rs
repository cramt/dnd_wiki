use crate::in_model::races::subrace::Subrace as In;
use crate::out_model::races::subrace::Subrace as Out;

impl From<In> for Out {
    fn from(x: In) -> Self {
        let In {
            name,
            features,
            flavor_text,
        } = x;
        Self {
            name,
            flavor_text,
            features,
        }
    }
}
