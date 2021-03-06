use crate::handlebars_engine::deserialize_context::deserialize_context;

use crate::out_model::class::Class;
use handlebars::{HelperDef, RenderError, ScopedJson};

#[allow(non_camel_case_types)]
pub struct features_at_level;

impl HelperDef for features_at_level {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let class = deserialize_context::<Class>(ctx)?.inner;
        let level = h
            .param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_i64()
            .ok_or_else(|| RenderError::new("not i64"))? as u8;
        let features = class.features.into_iter().flat_map(|(_, x)|x).filter(|x|x.relevant_levels.contains(&level)).collect::<Vec<_>>();
        let json =
            serde_json::value::to_value(features).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(json))
    }
}
