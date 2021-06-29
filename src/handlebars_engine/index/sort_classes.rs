use handlebars::{RenderError, HelperDef, ScopedJson};
use serde::de::IntoDeserializer;
use crate::out_model::class::Class;
use serde::Deserialize;

#[allow(non_camel_case_types)]
pub struct sort_classes;

impl HelperDef for sort_classes {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let mut claases = Vec::<Class>::deserialize(h.param(0)
            .ok_or(RenderError::new("param not found"))?
            .value()
            .clone()
            .into_deserializer()).map_err(|x| RenderError::new(x.to_string()))?;
        claases.sort_by(|a, b| a.name.cmp(&b.name));
        let json = serde_json::value::to_value(claases).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(
            json
        ))
    }
}
