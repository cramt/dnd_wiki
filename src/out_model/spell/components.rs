use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Components {
    verbal: bool,
    somatic: bool,
    material: Option<String>,
}
