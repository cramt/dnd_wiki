use handlebars::Template;

macro_rules! template_export {
    ($($name:ident),*) => {
        $(
            pub fn $name() -> Template {
                let mut template = Template::compile(include_str!(concat!("./", stringify!($name), ".hbs"))).unwrap();
                template.name = Some(stringify!($name).into());
                template
            }
        )*
    }
}

template_export!(class, index, spells, spell, subclass);
