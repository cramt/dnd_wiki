pub mod class;
pub mod deserialize_context;
pub mod global;
pub mod index;
pub mod render_err;
pub mod spell;
pub mod spells;

use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Engine<T: Serialize> {
    template_name: &'static str,
    inner: &'static Handlebars<'static>,
    t: PhantomData<T>,
}

impl<T: Serialize> Engine<T> {
    pub fn new(template_name: &'static str, inner: &'static Handlebars<'static>) -> Self {
        Self {
            template_name,
            inner,
            t: PhantomData,
        }
    }
}

impl<T: Serialize> Engine<T> {
    pub fn render(&self, obj: &T) -> Result<String, RenderError> {
        self.inner.render(self.template_name, obj)
    }
}

#[macro_export]
macro_rules! engine_generator {
    ($t:ty >> $template:ident >> $($name:ident),*) => {
        use handlebars::Handlebars;
        use std::ops::Deref;

        $(
            mod $name;
        )*

        pub fn engine() -> crate::handlebars_engine::Engine<crate::out_model::index::MetadataWrapper<$t>> {
            #[allow(unused_mut)]
            static LAZY: once_cell::sync::Lazy<Handlebars<'static>> = once_cell::sync::Lazy::new(|| {
                let mut reg = crate::handlebars_engine::global::global_engine();
                reg.register_template(stringify!($template), crate::handlebars_definitions::$template());
                $(
                    reg.register_helper(
                        stringify!($name),
                        Box::new($name::$name),
                    );
                )*
                reg
            });
            crate::handlebars_engine::Engine::new(stringify!($template), LAZY.deref())
        }
    }
}
