use handlebars::{HelperDef, JsonValue, RenderError, ScopedJson};
use serde_json::Number;

#[allow(non_camel_case_types)]
pub struct to_i;

impl HelperDef for to_i {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let a = h
            .param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_f64()
            .ok_or_else(|| RenderError::new("not f64"))?;

        Ok(ScopedJson::Derived(JsonValue::Number(Number::from(
            a.round() as i64,
        ))))
    }
}
