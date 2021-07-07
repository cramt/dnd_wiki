use handlebars::{HelperDef, RenderError, ScopedJson};

use crate::{
    handlebars_engine::{deserialize_context::deserialize_context},
    out_model::featlikes::Featlikes,
};

#[allow(non_camel_case_types)]
pub struct sort_feats;

impl HelperDef for sort_feats {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let mut arr = deserialize_context::<Featlikes>(ctx)?.inner.entries;
        arr.sort_by(|a,b|a.name.cmp(&b.name));
        let json = serde_json::value::to_value(arr).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(json))
    }
}
