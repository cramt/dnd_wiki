use crate::handlebars_engine::render_err::render_err;
use crate::out_model::spell::Spell;
use crate::text_utils::class_name_sanitize;
use handlebars::{HelperDef, JsonValue, RenderError, ScopedJson};
use serde::de::IntoDeserializer;
use serde::Deserialize;

#[allow(non_camel_case_types)]
pub struct classes_of_spell;

impl HelperDef for classes_of_spell {
    #[allow(unused_assignments)]
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &::handlebars::Helper<'reg, 'rc>,
        _: &'reg ::handlebars::Handlebars<'reg>,
        _: &'rc ::handlebars::Context,
        _: &mut ::handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<::handlebars::ScopedJson<'reg, 'rc>, ::handlebars::RenderError> {
        let spell = Spell::deserialize(
            h.param(0)
                .ok_or_else(|| RenderError::new("param not found"))?
                .value()
                .clone()
                .into_deserializer(),
        )
        .map_err(render_err)?;

        let mut vec = spell
            .casters
            .iter()
            .map(|x| format!("class-{}", class_name_sanitize(x)))
            .collect::<Vec<_>>();

        vec.push(format!("school-{}", class_name_sanitize(spell.school)));
        if spell.ritual {
            vec.push("ritual".to_string());
        }

        let json = JsonValue::String(vec.join(" "));
        Ok(ScopedJson::Derived(json))
    }
}
