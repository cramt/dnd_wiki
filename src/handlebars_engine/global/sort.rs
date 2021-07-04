use handlebars::{HelperDef, RenderError, ScopedJson};
use serde::{de::IntoDeserializer, Deserialize};

use crate::handlebars_engine::render_err::render_err;

#[allow(non_camel_case_types)]
pub struct sort;

impl HelperDef for sort {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let mut arr = Vec::<String>::deserialize(
            h.param(0)
                .ok_or_else(|| RenderError::new("param not found"))?
                .value()
                .clone()
                .into_deserializer(),
        )
        .map_err(render_err)?;
        arr.sort();
        let json = serde_json::value::to_value(arr).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(json))
    }
}
