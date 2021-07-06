use handlebars::{HelperDef, JsonValue, RenderError, ScopedJson};

#[allow(non_camel_case_types)]
pub struct or;

impl HelperDef for or {
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
            .as_bool()
            .ok_or_else(|| RenderError::new("not bool"))?;
        let b = h
            .param(1)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_bool()
            .ok_or_else(|| RenderError::new("not bool"))?;

        Ok(ScopedJson::Derived(JsonValue::Bool(a || b)))
    }
}
