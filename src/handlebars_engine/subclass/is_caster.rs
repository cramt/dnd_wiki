use crate::out_model::class::subclass::Subclass;
use crate::{
    handlebars_engine::deserialize_context::deserialize_context,
    out_model::class::generic_class::GenericClass,
};
use handlebars::{HelperDef, JsonValue, ScopedJson};

#[allow(non_camel_case_types)]
pub struct is_caster;

impl HelperDef for is_caster {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let class = deserialize_context::<Subclass>(ctx)?.inner;
        Ok(ScopedJson::Derived(JsonValue::Bool(class.has_spells())))
    }
}
