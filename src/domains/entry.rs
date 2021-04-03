use serde::{Serialize, Deserialize};
use std::{fs::File, path::Path, path::PathBuf};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Entry {
    id: String,
    title: String,
}

impl Entry {
    pub fn new(chapter: &str, entry_size: &usize, title: &str) -> Self {
        Entry {
            id: Self::generate_id(chapter, entry_size).clone(),
            title: String::from(title)
        }
    }

    pub fn file_name(&self) -> String {
        format!("./{}.txt", self.id)
    }

    pub fn init(&self, base_path: &Path) -> Result<Self, std::io::Error> {
        match File::create(self.entry_path(base_path)) {
            Ok(_) => Ok(self.clone()),
            Err(e) => Err(e)
        }
    }

    fn entry_path(&self, base_path: &Path) -> PathBuf {
        base_path.join(&self.id)
    }

    fn generate_id(chapter: &str ,entry_size: &usize) -> String {
        format!("{}-{}", chapter, entry_size + 1)
    }
}