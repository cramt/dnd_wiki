use crate::model::class::feature::Feature as In;
use crate::model::class::feature::Section as InSec;
use crate::out_model::class::feature::Feature as Out;
use crate::out_model::class::feature::Section as OutSec;

impl Into<OutSec> for InSec {
    fn into(self) -> OutSec {
        let InSec {
            name,
            body,
            sections,
        } = self;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        OutSec {
            name,
            body,
            sections,
        }
    }
}

impl Into<InSec> for OutSec {
    fn into(self) -> InSec {
        let OutSec {
            name,
            body,
            sections,
        } = self;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        InSec {
            name,
            body,
            sections,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        let Out {
            name,
            level,
            body,
            sections,
        } = self;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        In {
            level,
            name,
            body,
            sections,
        }
    }
}

impl Into<Out> for In {
    fn into(self) -> Out {
        let In {
            name,
            level,
            body,
            sections,
        } = self;
        let sections = sections.into_iter().map(|f| f.into()).collect();
        Out {
            level,
            name,
            body,
            sections,
        }
    }
}
