use shoulda::Shoulda;

use crate::in_model::races::{race::Race, race_category::RaceCategory, Races};

#[test]
fn aaa() {
    let a = r#"
race_categories:
  - cool 
races:
  - name: cool peps
    flavor_text: they cool
    category: cool
    cool: they cool
    subraces:
      - name: crazy cool
        crazy cool: they crazy cool
    "#;
    serde_yaml::from_str::<Races>(a)
        .unwrap()
        .should()
        .eq(Races {
            race_categories: vec![RaceCategory {
                name: "cool".to_string(),
                body: None,
            }],
            races: vec![Race {
                name: "cool peps".to_string(),
                flavor_text: Some("they cool".to_string()),
                category: "cool".to_string(),
                features: vec![("cool".to_string(), "they cool".to_string())]
                    .into_iter()
                    .collect(),
                subraces: vec![],
            }],
        });
}
