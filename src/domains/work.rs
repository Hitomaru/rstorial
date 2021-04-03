use std::{fs::File, io::{Read, Write}, path::PathBuf, usize};
use std::fs::DirBuilder;
use std::path::Path;
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
    entries: Vec<Entry>,
}


impl Work {
    pub fn new(id: &str,author: &str, title: &str, description: &str, base_path: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            description: description.to_string(),
            base_path: base_path.to_string(),
            entries: Vec::new()
        }
    }

    pub fn init(&self) -> Result<Work, std::io::Error> {
        Self::generate_dir_if_exists(&self.work_dir_path())?;
        self.create_description()?;
        Ok(self.clone())
    }

    pub fn work_dir_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(self.id.to_string())
    }

    pub fn description_path(&self) -> PathBuf {
        self.work_dir_path().join("description.yml")
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
        match File::open(path).and_then(|mut f| {f.read_to_string(&mut yaml)}) {
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

        log::debug!("{}", self.description_path().display());
        match File::create(self.description_path()) {
            Ok(mut f) => Ok(
                f.write(yaml.as_bytes())
                    .map(|_| {yaml})?),
            Err(e) => Err(e)
        }
    }

    pub fn add_entry(self, chapter: &str) -> Result<Work, std::io::Error> {
        let chapter_dir_path = self.work_dir_path().join("chapters");
        log::debug!("Creating chapter dir: {}", &chapter_dir_path.to_str().unwrap_or("File path cannot be extracted."));
        Self::generate_dir_if_exists(&chapter_dir_path)?;
        let entry = Entry::new(chapter, &self.entries.len(), "");
        let mut new_entries = self.entries.clone();
        match entry.init(chapter_dir_path.as_path()) {
            Ok(entry) => new_entries.push(entry),
            Err(e) => return Err(e)
        }
        let new_value = Self{
            entries: new_entries,
            ..self
        };
        Ok(new_value)
    }

    fn generate_dir_if_exists(path: &PathBuf) -> Result<(), std::io::Error> {
        if let Err(error) = DirBuilder::new().create(&path) {
            if error.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(error)
            }
        }
        Ok(())
    }
}

impl fmt::Display for Work {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, written by {}", self.title, self.author)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;
    use super::{Work};

    const TEST_BASE_DIR_PATH: &str = "./target/test";

    fn generate_reference_struct() -> Work {
        Work::new(
            "test_id",
            "test_author",
            "test_title",
            "test_description",
            TEST_BASE_DIR_PATH
        )
    }

    #[test]
    fn load_description_should_return_error_when_file_was_not_found() {
        let missing_file_path = Path::new("./this/description/file/is/missing");
        let sut = Work::load_description(missing_file_path).unwrap_err();
        let expected = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The description file was not found.");
        assert_eq!(sut.kind(), expected.kind());
    }

    #[test]
    fn load_description_should_load_description_file() {
        fs::create_dir_all(TEST_BASE_DIR_PATH).unwrap();
        let work = generate_reference_struct();
        work.init().unwrap();

        let sut = Work::load_description(work.description_path().as_path()).unwrap();
        let expected = work;
        
        fs::remove_dir_all(TEST_BASE_DIR_PATH).unwrap();
        assert_eq!(sut, expected);
    }

    #[test]
    fn description_path_returns_description_yml_path() {
        let work = generate_reference_struct();
        let sut = work.description_path().as_path().to_str().unwrap().to_string();
        let expected = format!("{}/description.yml", work.work_dir_path().to_str().unwrap());
        assert_eq!(sut, expected);
    }

    #[test]
    fn work_dir_path_returns_working_directory_path() {
        let work = generate_reference_struct();
        let sut = work.work_dir_path().as_path().to_str().unwrap().to_string();
        let expected = format!("{}/test_id", work.base_path.to_string());
        assert_eq!(sut, expected);
    }
}