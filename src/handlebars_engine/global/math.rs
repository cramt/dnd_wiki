use handlebars::{HelperDef, RenderError, ScopedJson, JsonValue};
use serde_json::Number;

#[allow(non_camel_case_types)]
pub struct math;

impl HelperDef for math {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let a = h.param(0).ok_or_else(|| RenderError::new("param not found"))?.value().as_f64().ok_or_else(|| RenderError::new("not f64"))?;
        let op = h.param(1).ok_or_else(|| RenderError::new("param not found"))?.value().as_str().ok_or_else(|| RenderError::new("not str"))?;
        let b = h.param(2).ok_or_else(|| RenderError::new("param not found"))?.value().as_f64().ok_or_else(|| RenderError::new("not f64"))?;

        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "%" => a % a,
            _ => return Err(RenderError::new("not mathematical operation"))
        };

        Ok(ScopedJson::Derived(
            JsonValue::Number(Number::from_f64(result).unwrap())
        ))
    }
}
