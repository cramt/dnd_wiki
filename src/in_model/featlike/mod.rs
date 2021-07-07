pub mod prerequisites;

use serde::Deserialize;

use self::prerequisites::Prerequisites;

#[derive(Debug, Deserialize)]
pub struct Featlike {
    pub name: String,
    #[serde(default)]
    pub prerequisites: Prerequisites,
    #[serde(default)]
    pub flavor_text: String,
    pub body: String,
}
