use std::{borrow::Cow, collections::HashMap};

use crate::referencer::Referencer;

pub struct HashMapReferencer {
    pub value: String,
    pub children: HashMap<String, HashMapReferencer>,
}

impl Referencer for HashMapReferencer {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        self.children.get(prop).map(|x| x as &dyn Referencer)
    }

    fn value(&self) -> Cow<str> {
        self.value.as_str().into()
    }
}