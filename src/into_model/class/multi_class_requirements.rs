use crate::in_model::class::multi_class_requirements::MultiClassRequirements as In;
use crate::in_model::class::multi_class_requirements::MulticlassRequirementKey as InKey;
use crate::out_model::class::multi_class_requirements::MultiClassRequirements as Out;
use crate::out_model::class::multi_class_requirements::MulticlassRequirementKey as OutKey;

impl From<InKey> for OutKey {
    fn from(val: InKey) -> Self {
        match val {
            InKey::Str => OutKey::Str,
            InKey::Dex => OutKey::Dex,
            InKey::Con => OutKey::Con,
            InKey::Int => OutKey::Con,
            InKey::Wis => OutKey::Wis,
            InKey::Cha => OutKey::Cha,
        }
    }
}

impl From<OutKey> for InKey {
    fn from(val: OutKey) -> Self {
        match val {
            OutKey::Str => InKey::Str,
            OutKey::Dex => InKey::Dex,
            OutKey::Con => InKey::Con,
            OutKey::Int => InKey::Con,
            OutKey::Wis => InKey::Wis,
            OutKey::Cha => InKey::Cha,
        }
    }
}

impl From<Out> for In {
    fn from(val: Out) -> Self {
        match val {
            Out::Value(a, b) => In::Value(a.into(), b),
            Out::And(b) => In::And(b.into_iter().map(|x| x.into()).collect()),
            Out::Or(b) => In::Or(b.into_iter().map(|x| x.into()).collect()),
        }
    }
}

impl From<In> for Out {
    fn from(val: In) -> Self {
        match val {
            In::Value(a, b) => Out::Value(a.into(), b),
            In::And(b) => Out::And(b.into_iter().map(|x| x.into()).collect()),
            In::Or(b) => Out::Or(b.into_iter().map(|x| x.into()).collect()),
        }
    }
}
