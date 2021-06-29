use crate::in_model::class::equipment::Equipment as In;
use crate::out_model::class::equipment::Equipment as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        Out(val.0)
    }
}

impl From<Out> for In {
    fn from(val: Out) -> Self {
        In(val.0)
    }
}
