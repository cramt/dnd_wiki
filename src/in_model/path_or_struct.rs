use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;

pub fn path_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    struct PathOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for PathOrStruct<T>
    where
        T: DeserializeOwned,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("path or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            serde_yaml::from_reader(crate::current_file::open_new(value))
                .map_err(|x| de::Error::custom(x.to_string()))
        }

        fn visit_seq<A>(self, seq: A) -> Result<T, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(PathOrStruct(PhantomData))
}
