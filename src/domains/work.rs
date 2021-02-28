use std::{fs::File, io::{Read, Write}, path::PathBuf, usize};
use std::fs::DirBuilder;
use std::path::Path;
use std::io::prelude;
use std::fmt;
use serde::{Serialize, Deserialize};

use super::entry::Entry;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Work{
    id: String,
    title: String,
    author: String,
    description: String,
    base_path: String,
    episodes: Vec<Entry>,
}


impl Work {
    pub fn new(id: &str,author: &str, title: &str, description: &str, base_path: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            description: description.to_string(),
            base_path: base_path.to_string(),
            episodes: Vec::new()
        }
    }

    pub fn init(&self) -> Result<String, std::io::Error> {
        self.create_work_dir()?;

        self.create_description()
    }

    pub fn work_dir_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(self.id.to_string())
    }

    pub fn description_path(&self) -> PathBuf {
        self.work_dir_path().join("description.yml")
    }

    fn create_work_dir(&self) -> std::io::Result<()> {
        DirBuilder::new()
            .create(self.work_dir_path())
    }

    pub fn load_description(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "The description file was not found."))
            }
        if path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "This file is a directory, not a file."))
        }

        let mut yaml = String::new();
        match File::open(path).map(|mut f| {f.read_to_string(&mut yaml)}) {
            Ok(_) => Ok(serde_yaml::from_str(&yaml).unwrap()),
            Err(e) => Err(e)
        }
    }

    pub fn create_description(&self) -> Result<String, std::io::Error> {
        if self.description_path().exists() { return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "The description file is already exists.")) }

        let yaml = serde_yaml::to_string(self).unwrap_or(String::from(""));
        match File::create(self.description_path()) {
            Ok(mut f) => Ok(f.write(yaml.as_bytes()).map(|_| {yaml})?),
            Err(e) => Err(e)
        }
    }

    pub fn save_description(&self) -> Result<String, std::io::Error> {
        let yaml = serde_yaml::to_string(self).unwrap_or(String::from(""));
        match File::open(self.description_path()) {
            Ok(mut f) => Ok(f.write(yaml.as_bytes()).map(|_| {yaml})?),
            Err(e) => Err(e)
        }
    }

    pub fn add_episode(self, chapter: &str) -> Result<Work, std::io::Error> {
        let chapter_dir_path = self.work_dir_path().join(chapter);
        DirBuilder::new().create(&chapter_dir_path)?;
        let entry = Entry::new(self.generate_chapter_id(chapter).as_str(), "");
        let episode_path = chapter_dir_path.join(self.episodes.len().to_string());
        File::create(episode_path)?;
        let mut new_episodes = self.episodes.clone();
        new_episodes.push(entry);
        let new_value = Self{
            episodes: new_episodes,
            ..self
        };
        Ok(new_value)
    }

    fn generate_chapter_id(&self, chapter: &str) -> String {
        format!("{}-{}", chapter,self.episodes.len())
    }
}

impl fmt::Display for Work {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, written by {}", self.title, self.author)
    }
}