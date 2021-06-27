use handlebars::{HelperDef, RenderError, ScopedJson};

#[allow(non_camel_case_types)]
pub struct range;

impl HelperDef for range {
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
            .as_u64()
            .ok_or_else(|| RenderError::new("param not u64"))? as u8;
        let b = match h.param(1) {
            None => None,
            Some(x) => Some(x.value().as_u64().ok_or_else(|| RenderError::new("param not u64"))? as u8)
        };
        let (a, b) = match b {
            None => (0, a),
            Some(b) => (a, b)
        };
        let b = b + 1;
        let json = serde_json::value::to_value((a..b).into_iter().collect::<Vec<u8>>()).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(
            json
        ))
    }
}
