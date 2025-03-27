use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassEntry {
    class: String,
    subclasses: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterTOML {
    subclasses: Vec<ClassEntry>,
}

impl CharacterTOML {
    pub fn iter(&self) -> impl Iterator<Item = &ClassEntry> {
        self.subclasses.iter()
    }
}
