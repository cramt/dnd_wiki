pub mod featlike;
pub mod prerequisites;
use serde::{Deserialize, Serialize};

use self::featlike::Featlike;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Featlikes {
    pub name: String,
    pub entries: Vec<Featlike>,
}
