use crate::out_model::class::feature::Feature;
use crate::out_model::class::Class;
use handlebars::{HelperDef, RenderError, ScopedJson};
use serde::de::IntoDeserializer;
use serde::Deserialize;

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
        let class = Class::deserialize(ctx.data().clone().into_deserializer())
            .map_err(|x| RenderError::new(x.to_string()))?;
        let level = h
            .param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_i64()
            .ok_or_else(|| RenderError::new("not i64"))? as u8;
        let features = class
            .features
            .into_iter()
            .filter(|x| x.level == level)
            .collect::<Vec<Feature>>();
        let json =
            serde_json::value::to_value(features).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(json))
    }
}
