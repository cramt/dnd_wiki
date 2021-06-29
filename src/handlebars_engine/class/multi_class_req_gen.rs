use crate::out_model::class::multi_class_requirements::MultiClassRequirements;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::collections::HashMap;

pub fn multi_class_req_gen(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).ok_or(RenderError::new("param not found"))?;
    let req = MultiClassRequirements::deserialize(param.value().clone().into_deserializer())?.and_vec();

    out.write(
        format!(
            "you must have {} in order to multiclass in or out of this class",
            req.into_iter()
                .fold(HashMap::new(), |mut acc, x| match x {
                    MultiClassRequirements::Value(a, b) => {
                        if !acc.contains_key(&b) {
                            acc.insert(b, Vec::new());
                        }
                        acc.get_mut(&b).unwrap().push(a.to_string());
                        acc
                    }
                    MultiClassRequirements::And(_) => unimplemented!(),
                    MultiClassRequirements::Or(_) => unimplemented!(),
                })
                .into_iter()
                .map(|(size, attr)| format!(
                    "{} of {} or higher",
                    attr.into_iter()
                        .map(|x| format!("a {} score", x))
                        .collect::<Vec<String>>()
                        .join(" and "),
                    size
                ))
                .collect::<Vec<String>>()
                .join(" and ")
        )
        .as_str(),
    )?;
    Ok(())
}
