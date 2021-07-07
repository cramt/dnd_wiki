use crate::handlebars_engine::render_err::render_err;
use crate::out_model::featlikes::featlike::Featlike;

use handlebars::{HelperDef, JsonValue, RenderError, ScopedJson};
use serde::de::IntoDeserializer;
use serde::Deserialize;

#[allow(non_camel_case_types)]
pub struct format_prerequisites;

impl HelperDef for format_prerequisites {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let entry = Featlike::deserialize(
            h.param(0)
                .ok_or_else(|| RenderError::new("param not found"))?
                .value()
                .clone()
                .into_deserializer(),
        )
        .map_err(render_err)?;
        Ok(ScopedJson::Derived(JsonValue::Bool(entry.prerequisites.exists())))
    }
}