use handlebars::{RenderError, HelperDef, ScopedJson, JsonValue};
use serde::de::IntoDeserializer;
use crate::model::class::Class;
use serde::Deserialize;
use crate::model::class::caster_type::CasterType;

#[allow(non_camel_case_types)]
pub struct has_spells;

impl HelperDef for has_spells {
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
            JsonValue::Bool(class.caster_type != CasterType::None)
        ))
    }
}
