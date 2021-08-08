use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

pub fn vec_or_single_element<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum OneOrMany<T> {
        One(T),
        Vec(Vec<T>),
    }
    impl<T> From<OneOrMany<T>> for Vec<T> {
        fn from(from: OneOrMany<T>) -> Self {
            match from {
                OneOrMany::One(val) => vec![val],
                OneOrMany::Vec(vec) => vec,
            }
        }
    }

    OneOrMany::deserialize(deserializer).map(|x| x.into())
}
