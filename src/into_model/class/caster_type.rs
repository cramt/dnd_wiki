use crate::out_model::class::caster_type::CasterType as Out;
use crate::model::class::caster_type::CasterType as In;

impl Into<Out> for In {
    fn into(self) -> Out {
        match self {
            In::Full => Out::Full,
            In::Half => Out::Half,
            In::Artificer => Out::Artificer,
            In::None => Out::None,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        match self {
            Out::Full => In::Full,
            Out::Half => In::Full,
            Out::Artificer => In::Full,
            Out::None => In::None,
        }
    }
}