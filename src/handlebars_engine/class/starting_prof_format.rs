use crate::out_model::class::starting_prof::StartingProfEntry;
use crate::text_utils::num_to_word;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::de::IntoDeserializer;
use serde::Deserialize;

pub fn starting_prof_format(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let prof = StartingProfEntry::deserialize(
        h.param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .clone()
            .into_deserializer(),
    )?;

    out.write(
        match prof {
            StartingProfEntry::StrictSet(set, anys) => format!(
                "{}{}",
                set.join(", "),
                if anys != 0 {
                    format!(". Choose any {}", num_to_word(anys))
                } else {
                    "".to_string()
                }
            ),
            StartingProfEntry::ChooseSet(amount, set, anys) => format!(
                "Choose any {} from {}{}",
                num_to_word(amount),
                set.join(", "),
                if anys != 0 {
                    format!(". Choose any {}", num_to_word(anys))
                } else {
                    "".to_string()
                }
            ),
            StartingProfEntry::Empty => "None".to_string(),
        }
        .as_str(),
    )?;
    Ok(())
}
