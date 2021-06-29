use crate::in_model::class::resource::Resource as In;
use crate::out_model::class::resource::Resource as Out;

impl Into<Out> for In {
    fn into(self) -> Out {
        Out(self.0)
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        In(self.0)
    }
}
