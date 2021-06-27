use handlebars::RenderError;
use serde::de::IntoDeserializer;
use crate::model::class::Class;
use serde::Deserialize;
use crate::model::class::caster_type::CasterType;
use handlebars::{HelperDef, ScopedJson, JsonValue};
use serde_json::Number;


#[allow(non_camel_case_types)]
pub struct max_spell_slot;

impl HelperDef for max_spell_slot {
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
        let slot = match class.caster_type {
            CasterType::Full => 9,
            CasterType::Half => 5,
            CasterType::Artificer => 5,
            CasterType::None => 0
        };
        Ok(ScopedJson::Derived(
            JsonValue::Number(Number::from(slot))
        ))
    }
}
