use crate::in_model::class::resource::Resource as In;
use crate::out_model::class::resource::Resource as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        Out(val.0)
    }
}
