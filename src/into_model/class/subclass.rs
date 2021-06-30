use crate::in_model::class::subclass::Subclass as In;
use crate::out_model::class::subclass::Subclass as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            name,
            flavour_text,
            features,
        } = val;
        let features = features.into_iter().map(|x| x.into()).collect();
        Out {
            name,
            features,
            flavour_text,
        }
    }
}
