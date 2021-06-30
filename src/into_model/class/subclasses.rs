use crate::in_model::class::subclasses::Subclasses as In;
use crate::out_model::class::feature::Feature;
use crate::out_model::class::subclasses::Subclasses as Out;

impl From<In> for (Out, Feature) {
    fn from(val: In) -> Self {
        let In {
            name,
            level,
            prefix,
            postfix,
            entries,
            features,
        } = val;
        let entries = entries.into_iter().map(|x| x.into()).collect();
        (
            Out {
                name: name.to_string(),
                features,
                entries,
            },
            Feature {
                name,
                level,
                body: format!("{}\n{}", prefix, postfix),
                sections: Vec::new(),
            },
        )
    }
}
