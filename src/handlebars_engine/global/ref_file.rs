use std::collections::HashMap;


use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;

static REF_FILE_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    static REF_FILES: Dir = include_dir!("./src/handlebars_definitions/ref_files");
    let mut map = HashMap::new();
    fn register_dir<'reg, 'a>(registry: &mut HashMap<String, String>, dir: &'a Dir) {
        for file in dir.files() {
            let content = file.contents_utf8().unwrap().replace("\\", "/");
            let name = file.path.replace("\\", "/");
            registry.insert(name, content);
        }
        for dir in dir.dirs() {
            register_dir(registry, dir)
        }
    }
    register_dir(&mut map, &REF_FILES);
    map
});

pub fn ref_file(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let name = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("not f64"))?;
    let content = REF_FILE_MAP
        .get(name)
        .ok_or_else(|| RenderError::new(format!("no files named {}", name)))?;
    out.write(content.as_str())?;
    Ok(())
}
