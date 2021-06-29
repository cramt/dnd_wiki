use crate::out_model::class::caster_type::CasterType;
use crate::out_model::class::Class;
use handlebars::RenderError;
use handlebars::{HelperDef, JsonValue, ScopedJson};
use serde::de::IntoDeserializer;
use serde::Deserialize;
use serde_json::Number;

#[allow(non_camel_case_types)]
pub struct max_spell_slot;

impl HelperDef for max_spell_slot {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let class = Class::deserialize(ctx.data().clone().into_deserializer())
            .map_err(|x| RenderError::new(x.to_string()))?;
        let slot = match class.caster_type {
            CasterType::Full => 9,
            CasterType::Half => 5,
            CasterType::Artificer => 5,
            CasterType::None => 0,
        };
        Ok(ScopedJson::Derived(JsonValue::Number(Number::from(slot))))
    }
}
