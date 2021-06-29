use crate::out_model::spell::Spell;
use handlebars::{HelperDef, RenderError, ScopedJson};
use serde::de::IntoDeserializer;
use serde::Deserialize;

#[allow(non_camel_case_types)]
pub struct sort_spells;

impl HelperDef for sort_spells {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        ctx: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let spells = Vec::<Spell>::deserialize(ctx.data().clone().into_deserializer())
            .map_err(|x| RenderError::new(x.to_string()))?;
        let level = h
            .param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .as_u64()
            .ok_or_else(|| RenderError::new("param not u64"))? as u8;
        let mut spells = spells
            .into_iter()
            .filter(|x| x.spell_level == level)
            .collect::<Vec<Spell>>();
        spells.sort_by(|a, b| a.name.cmp(&b.name));
        let json =
            serde_json::value::to_value(spells).map_err(|x| RenderError::new(x.to_string()))?;
        Ok(ScopedJson::Derived(json))
    }
}
