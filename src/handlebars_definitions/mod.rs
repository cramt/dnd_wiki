use handlebars::Template;

macro_rules! template_export {
    ($($name:ident),*) => {
        $(
            pub fn $name() -> Template {
                Template::compile(include_str!(concat!("./", stringify!($name), ".hbs"))).unwrap()
            }
        )*
    }
}

template_export!(class, index, spells, spell);
