use crate::model::spell::components::Components as In;
use crate::out_model::spell::components::Components as Out;

impl Into<Out> for In {
    fn into(self) -> Out {
        let Self {
            verbal,
            somatic,
            material,
        } = self;
        Out {
            verbal,
            somatic,
            material,
        }
    }
}


impl Into<In> for Out {
    fn into(self) -> In {
        let Self {
            verbal,
            somatic,
            material,
        } = self;
        In {
            verbal,
            somatic,
            material,
        }
    }
}
