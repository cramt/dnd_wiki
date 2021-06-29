use crate::in_model::class::feature::Feature as In;
use crate::in_model::class::feature::Section as InSec;
use crate::out_model::class::feature::Feature as Out;
use crate::out_model::class::feature::Section as OutSec;

impl From<InSec> for OutSec {
    fn from(val: InSec) -> Self {
        let InSec {
            name,
            body,
            sections,
        } = val;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        OutSec {
            name,
            body,
            sections,
        }
    }
}

impl From<OutSec> for InSec {
    fn from(val: OutSec) -> Self {
        let OutSec {
            name,
            body,
            sections,
        } = val;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        InSec {
            name,
            body,
            sections,
        }
    }
}

impl From<Out> for In {
    fn from(val: Out) -> Self {
        let Out {
            name,
            level,
            body,
            sections,
        } = val;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        In {
            level,
            name,
            body,
            sections,
        }
    }
}

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            name,
            level,
            body,
            sections,
        } = val;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        Out {
            level,
            name,
            body,
            sections,
        }
    }
}
