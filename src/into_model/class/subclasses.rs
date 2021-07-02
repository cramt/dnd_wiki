use crate::in_model::class::subclasses::Subclasses as In;
use crate::out_model::class::feature::Feature;
use crate::out_model::class::subclass::Subclass;
use crate::out_model::class::subclasses::Subclasses as Out;

impl In {
    pub fn out(self, class_name: &str) -> (Out, Feature) {
        let In {
            name,
            level,
            prefix,
            postfix,
            entries,
            features,
        } = self;
        let entries: Vec<Subclass> = entries.into_iter().map(|x| x.into()).collect();
        let f = Feature {
            name: name.to_string(),
            level,
            body: format!(
                "{}\r\n{}{}",
                prefix,
                entries
                    .iter()
                    .map(|x| format!(". [[{}.{}]]\r\n", class_name, x.name))
                    .collect::<String>(),
                postfix
            ),
            sections: Vec::new(),
        };
        (
            Out {
                name,
                features,
                entries,
            },
            f,
        )
    }
}