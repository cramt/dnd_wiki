use handlebars::{HelperDef, JsonValue, RenderError, ScopedJson};
use serde_json::Number;

#[allow(non_camel_case_types)]
pub struct prof_bonus;

impl HelperDef for prof_bonus {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let level = h
            .param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_f64()
            .ok_or_else(|| RenderError::new("not f64"))?;

        Ok(ScopedJson::Derived(JsonValue::Number(
            Number::from_f64((level / 4f64).ceil() + 1f64).unwrap(),
        )))
    }
}
