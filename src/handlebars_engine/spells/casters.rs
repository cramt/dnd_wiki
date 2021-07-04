use std::collections::HashSet;

use crate::handlebars_engine::deserialize_context::deserialize_context;
use crate::handlebars_engine::render_err::render_err;
use crate::out_model::spell::Spell;
use handlebars::{HelperDef, ScopedJson};

#[allow(non_camel_case_types)]
pub struct casters;

impl HelperDef for casters {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let spells = deserialize_context::<Vec<Spell>>(ctx)?.inner;
        let lists = spells
            .into_iter()
            .map(|x| x.casters)
            .flatten()
            .collect::<HashSet<_>>();

        let json =
            serde_json::value::to_value(lists).map_err(render_err)?;
        Ok(ScopedJson::Derived(json))
    }
}
