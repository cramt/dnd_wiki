use handlebars::{RenderError, HelperDef, ScopedJson, JsonValue};
use serde::de::IntoDeserializer;
use crate::out_model::class::Class;
use serde::Deserialize;


#[allow(non_camel_case_types)]
pub struct has_cantrips;

impl HelperDef for has_cantrips {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let class = Class::deserialize(h.param(0)
            .ok_or(RenderError::new("param not found"))?
            .value()
            .clone()
            .into_deserializer()).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(
            JsonValue::Bool(class.start_cantrips_known.is_some())
        ))
    }
}
