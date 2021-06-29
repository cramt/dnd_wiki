use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
}

impl ToString for Components {
    fn to_string(&self) -> String {
        let mut v = Vec::with_capacity(
            self.verbal as usize + self.somatic as usize + self.material.is_some() as usize,
        );
        if self.verbal {
            v.push("V".to_string());
        }
        if self.somatic {
            v.push("S".to_string());
        }
        if let Some(m) = self.material.as_ref() {
            v.push(format!("M ({})", m));
        }
        v.join(", ")
    }
}