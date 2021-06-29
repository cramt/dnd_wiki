use crate::in_model::class::caster_type::CasterType as In;
use crate::out_model::class::caster_type::CasterType as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        match val {
            In::Full => Out::Full,
            In::Half => Out::Half,
            In::Artificer => Out::Artificer,
            In::None => Out::None,
        }
    }
}

impl From<Out> for In {
    fn from(val: Out) -> Self {
        match val {
            Out::Full => In::Full,
            Out::Half => In::Full,
            Out::Artificer => In::Full,
            Out::None => In::None,
        }
    }
}
