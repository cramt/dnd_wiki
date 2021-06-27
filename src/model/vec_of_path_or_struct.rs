use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::{de, Deserializer};
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
            let mut v = Vec::new();
            loop {
                let str_result = seq.next_element::<String>()?;
                let obj_result = seq.next_element::<T>()?;
                let x = match (str_result, obj_result) {
                    (Some(str), _) => serde_yaml::from_reader(crate::current_file::open_new(str))
                        .map_err(|x| de::Error::custom(x.to_string()))?,
                    (_, Some(obj)) => obj,
                    _ => break,
                };
                v.push(x)
            }
            Ok(v)
        }
    }

    deserializer.deserialize_any(VecOfPathOrStruct(PhantomData))
}
