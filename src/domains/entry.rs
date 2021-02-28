use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Entry {
    id: String,
    title: String,
}

impl Entry {
    pub fn new(id: &str, title: &str) -> Self {
        Entry {
            id: String::from(id),
            title: String::from(title)
        }
    }

    pub fn file_name(&self) -> String {
        format!("./{}.txt", self.id)
    }
}