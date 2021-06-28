use std::fmt::Formatter;
use core::fmt;
use serde::de::{SeqAccess, Visitor, MapAccess};
use serde::{Deserialize, Deserializer, de, Serialize};
use std::mem::MaybeUninit;

#[derive(Debug, Serialize)]
pub struct Resource([String; 20]);

#[derive(Debug, Deserialize)]
struct CalcResource {
    #[serde(default)]
    postfix: String,
    #[serde(default)]
    prefix: String,
    formula: String,
}

impl CalcResource {
    pub fn into_resource(self) -> Result<Resource, String> {
        let expr = self.formula.parse::<meval::Expr>().map_err(|x| x.to_string())?;
        let mut array: [MaybeUninit<String>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
        for (level, element) in array.iter_mut().enumerate().map(|(a, b)| (a + 1, b)) {
            let mut ctx = meval::Context::new();
            ctx.var("level", level as f64);
            *element = MaybeUninit::new(format!("{}{}{}", self.prefix, expr.eval_with_context(ctx).map_err(|x| x.to_string())?, self.postfix));
        }
        let arr = unsafe {std::mem::transmute::<_, [String; 20]>(array)};
        Ok(Resource(arr))
    }
}

struct ResourceVisitor;

impl<'de> Visitor<'de> for ResourceVisitor {
    type Value = Resource;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("standard class resource")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error> where
        A: SeqAccess<'de>, {
        let arr: [String; 20] = Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
        Ok(Resource(arr))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error> where
        A: MapAccess<'de>, {
        CalcResource::deserialize(de::value::MapAccessDeserializer::new(map))?.into_resource().map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ResourceVisitor)
    }
}
