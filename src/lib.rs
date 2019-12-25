use std::{fs, path::Path};

pub mod error;
mod pathfinder;
mod util;

pub use pathfinder::PathFinder;

/// Wrapper around `yaml_rust::Yaml` that implements `PathFinder`
pub struct Yaml(yaml_rust::Yaml);

impl std::ops::Deref for Yaml {
    type Target = yaml_rust::Yaml;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Yaml {
    /// Conveniently open a yaml file
    pub fn open<P: AsRef<Path>>(path: &P) -> Result<Yaml, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(&path)?;
        Self::parse(&file_content)
    }

    pub fn parse(file_content: &str) -> Result<Yaml, Box<dyn std::error::Error>> {
        util::parse(file_content).map(Self)
    }
}
