use crate::in_model::spell::components::Components as In;
use crate::out_model::spell::components::Components as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            verbal,
            somatic,
            material,
        } = val;
        Out {
            verbal,
            somatic,
            material,
        }
    }
}
