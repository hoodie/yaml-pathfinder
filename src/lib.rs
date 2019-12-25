use yaml_rust::YamlLoader;
pub use yaml_rust::Yaml;

#[allow(unused_imports)]
use std::{
    fmt,
    path::Path,
    {ffi::OsStr, fs},
};

pub mod error;
mod pathfinder;
mod util;

pub use pathfinder::PathFinder;
/*
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
    pub fn open<P: AsRef<OsStr> + Sized>(path: P) -> Result<Yaml, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path.as_ref())?;
        Self::parse(&file_content)
    }

    pub fn parse(file_content: &str) -> Result<Yaml, Box<dyn std::error::Error>> {
        util::parse(file_content).map(Self)
    }
}

impl PathFinder for Yaml {
    fn data(&self) -> &yaml_rust::Yaml {
        &self.0
    }
}

impl fmt::Debug for Yaml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
*/

/// conviniently just opens and parses a `.yml` file.
pub fn open_yaml<P: AsRef<OsStr> + Sized>(path: P) -> Result<Yaml, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(path.as_ref())?;
    parse_yaml(&file_content)
}

/// Ruby like API to yaml-rust.
pub fn parse_yaml(file_content: &str) -> Result<yaml_rust::Yaml, Box<dyn std::error::Error>> {
    Ok(YamlLoader::load_from_str(&file_content)?
        .get(0)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| yaml_rust::Yaml::from_str("[]")))
}
