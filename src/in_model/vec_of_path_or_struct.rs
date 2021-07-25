use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;

pub fn vec_of_path_or_struct<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    struct VecOfPathOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for VecOfPathOrStruct<T>
    where
        T: DeserializeOwned,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("seq of path or map")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
        where
            A: SeqAccess<'de>,
        {
            enum Entry<K> {
                Loaded(K),
                Lazy(String),
            }
            struct EntryVisitor<K>(PhantomData<fn() -> K>);
            impl<'de, K> Visitor<'de> for EntryVisitor<K>
            where
                K: Deserialize<'de>,
            {
                type Value = Entry<K>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("path or map")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    self.visit_string(v.to_string())
                }

                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Entry::Lazy(v))
                }

                fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    Ok(Entry::Loaded(Deserialize::deserialize(
                        serde::de::value::SeqAccessDeserializer::new(seq),
                    )?))
                }

                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    Ok(Entry::Loaded(Deserialize::deserialize(
                        serde::de::value::MapAccessDeserializer::new(map),
                    )?))
                }
            }

            impl<'de, K> Deserialize<'de> for Entry<K>
            where
                K: Deserialize<'de>,
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_any(EntryVisitor::<K>(PhantomData::default()))
                }
            }

            let mut v: Vec<Entry<T>> = Vec::new();
            while let Some(entry) = seq.next_element::<Entry<T>>()? {
                v.push(entry);
            }
            Ok(v.into_iter()
                .map(|x| match x {
                    Entry::Loaded(x) => x,
                    Entry::Lazy(str) => {
                        serde_yaml::from_reader(crate::current_file::open_new(str)).unwrap()
                    }
                })
                .collect())
        }
    }

    deserializer.deserialize_seq(VecOfPathOrStruct(PhantomData))
}
