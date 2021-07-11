use std::{borrow::Cow,  str::from_utf8};

pub trait Referencer {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer>;
    fn value(&self) -> Cow<str>;
    fn traverse<T: AsRef<str>, I: Iterator<Item = T>>(&self, iter: I) -> Option<&dyn Referencer>
    where
        Self: Sized,
    {
        let mut i = self as &dyn Referencer;
        for x in iter {
            i = i.prop(x.as_ref())?;
        }
        Some(i)
    }
}


impl<T> Referencer for &T where T: Referencer {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        T::prop(&self, prop)
    }

    fn value(&self) -> Cow<str> {
        T::value(&self)
    }
}

impl<const N: usize> Referencer for [u8; N] {
    fn prop(&self, _: &str) -> Option<&dyn Referencer> {
        None
    }

    fn value(&self) -> Cow<str> {
        from_utf8(self).unwrap().into()
    }
}