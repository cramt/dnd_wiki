use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::Deserializer;
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
            let mut v: Vec<Entry<T>> = Vec::new();
            loop {
                let str_result = seq.next_element::<String>();
                if let Ok(Some(str)) = str_result {
                    v.push(Entry::Lazy(str));
                    continue;
                }
                let obj_result = seq.next_element::<T>();
                if let Ok(Some(obj)) = obj_result {
                    v.push(Entry::Loaded(obj));
                    continue;
                }
                if obj_result.is_err() && str_result.is_err() {
                    obj_result?;
                    str_result?;
                }
                break;
            }
            Ok(v.into_iter().map(|x| match x {
                Entry::Loaded(x) => x,
                Entry::Lazy(str) => serde_yaml::from_reader(crate::current_file::open_new(str)).unwrap()
            }).collect())
        }
    }

    deserializer.deserialize_seq(VecOfPathOrStruct(PhantomData))
}
