macro_rules! global_engine_gen {
    ($($name:ident),*) => {
        use handlebars::Handlebars;

        $(
            mod $name;
        )*

        pub fn global_engine<'reg>() -> Handlebars<'reg>{
            let mut reg = Handlebars::new();
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

global_engine_gen!(range, proper_noun, spell_level_name, ordinal, prof_bonus);
