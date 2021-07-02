use crate::handlebars_engine::deserialize_context::deserialize_context;
use crate::out_model::class::caster_type::CasterType;
use crate::out_model::class::Class;
use handlebars::{HelperDef, JsonValue, ScopedJson};

#[allow(non_camel_case_types)]
pub struct has_spells;

impl HelperDef for has_spells {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let class = deserialize_context::<Class>(ctx)?.inner;
        Ok(ScopedJson::Derived(JsonValue::Bool(
            class.caster_type != CasterType::None
        )))
    }
}
