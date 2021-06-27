mod class;
mod index;
mod spells;
mod spell;
mod global;

use crate::model::index::Index;
use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! engine_generator {
    ($($name:ident),*) => {
        use handlebars::Handlebars;

        $(
            mod $name;
        )*

        pub fn new_engine<'reg>() -> Handlebars<'reg> {
            #[allow(unused_mut)]
            let mut reg = crate::handlebars_engine::global::global_engine();
            $(
                reg.register_helper(
                    stringify!($name),
                    Box::new($name::$name),
                );
            )*
            reg
        }
    }
}

pub fn build(index: &Index) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut map = HashMap::new();
    map.insert("index.html".to_string(),  index::new_engine().render_template(super::handlebars_definitions::index(), &index)?);
    map.insert("spells/index.html".to_string(), spells::new_engine().render_template(super::handlebars_definitions::spells(), &index.spells)?);
    for spell in &index.spells {
        map.insert(format!("spells/{}.html", spell.name), spell::new_engine().render_template(super::handlebars_definitions::spell(), spell)?);
    }
    for class in &index.classes {
        map.insert(format!("classes/{}.html", class.name), class::new_engine().render_template(super::handlebars_definitions::class(), class)?);
    }
    Ok(map)
}
