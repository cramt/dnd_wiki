use crate::model::class::multi_class_requirements::MultiClassRequirements as In;
use crate::model::class::multi_class_requirements::MulticlassRequirementKey as InKey;
use crate::out_model::class::multi_class_requirements::MultiClassRequirements as Out;
use crate::out_model::class::multi_class_requirements::MulticlassRequirementKey as OutKey;

impl Into<OutKey> for InKey {
    fn into(self) -> OutKey {
        match self {
            InKey::Str => OutKey::Str,
            InKey::Dex => OutKey::Dex,
            InKey::Con => OutKey::Con,
            InKey::Int => OutKey::Con,
            InKey::Wis => OutKey::Wis,
            InKey::Cha => OutKey::Cha,
        }
    }
}

impl Into<InKey> for OutKey {
    fn into(self) -> InKey {
        match self {
            OutKey::Str => InKey::Str,
            OutKey::Dex => InKey::Dex,
            OutKey::Con => InKey::Con,
            OutKey::Int => InKey::Con,
            OutKey::Wis => InKey::Wis,
            OutKey::Cha => InKey::Cha,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        match self {
            Out::Value(a, b) => In::Value(a.into(), b),
            Out::And(b) => In::And(b.into_iter().map(|x| x.into()).collect()),
            Out::Or(b) => In::Or(b.into_iter().map(|x| x.into()).collect()),
        }
    }
}

impl Into<Out> for In {
    fn into(self) -> Out {
        match self {
            In::Value(a, b) => Out::Value(a.into(), b),
            In::And(b) => Out::And(b.into_iter().map(|x| x.into()).collect()),
            In::Or(b) => Out::Or(b.into_iter().map(|x| x.into()).collect()),
        }
    }
}
