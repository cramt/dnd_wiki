use crate::handlebars_engine::deserialize_context::deserialize_context;
use crate::out_model::class::Class;
use handlebars::{HelperDef, JsonValue, ScopedJson};
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
        let class = deserialize_context::<Class>(ctx)?.inner;
        let slot = class.caster_type.max_spell_slot();
        Ok(ScopedJson::Derived(JsonValue::Number(Number::from(slot))))
    }
}
