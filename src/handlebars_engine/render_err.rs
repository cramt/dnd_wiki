use std::error::Error;

use handlebars::RenderError;

pub fn render_err<T: Error>(t: T) -> RenderError {
    RenderError::new(t.to_string())
}
