pub mod race;
pub mod subrace;

use std::collections::HashMap;

use crate::in_model::races::Races as In;
use crate::out_model::races::race::Race;
use crate::out_model::races::Races as Out;

impl From<In> for Out {
    fn from(x: In) -> Self {
        let In {
            race_categories,
            races,
        } = x;
        let mut map = race_categories
            .into_iter()
            .map(|x| (x.name, (x.body, Vec::new())))
            .collect::<HashMap<String, (Option<String>, Vec<Race>)>>();
        for race in races {
            map.get_mut(&race.category)
                .ok_or_else(|| format!("{} is not a race category", race.category))
                .unwrap()
                .1
                .push(race.into())
        }
        Self(map)
    }
}
