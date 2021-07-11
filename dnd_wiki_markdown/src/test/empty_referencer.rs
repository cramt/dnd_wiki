use std::borrow::Cow;

use crate::referencer::Referencer;

pub struct EmptyReferencer;

impl Referencer for EmptyReferencer {
    fn prop(&self, _: &str) -> Option<&dyn Referencer> {
        None
    }

    fn value(&self) -> Cow<str> {
        "".into()
    }
}