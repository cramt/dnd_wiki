use std::{fmt::Debug, intrinsics::transmute, marker::PhantomData, ops::Deref};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[derive(Debug)]
pub struct Crs<T> {
    inner: usize,
    t: PhantomData<T>,
}

impl<T> Clone for Crs<T> {
    fn clone(&self) -> Self {
        Self::inner_new(self.inner)
    }
}

impl<T> Crs<T> {
    fn inner_new(n: usize) -> Self {
        Self {
            inner: n,
            t: PhantomData,
        }
    }

    pub fn new(t: &T) -> Self {
        let raw: *const usize = unsafe { transmute(t) };
        Self::inner_new(raw as usize)
    }
}

impl<T> Deref for Crs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let raw = self.inner as *const usize;
        unsafe { &*(raw as *const T) }
    }
}

struct UsizeVisitor;

impl<'de> Visitor<'de> for UsizeVisitor {
    type Value = usize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("mem address in form of u32")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v as usize)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v as usize)
    }
}

impl<'de, T> Deserialize<'de> for Crs<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::inner_new(deserializer.deserialize_u32(UsizeVisitor)?))
    }
}

impl<T> Serialize for Crs<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.inner as u64)
    }
}
