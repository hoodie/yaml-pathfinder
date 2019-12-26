pub use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

#[allow(unused_imports)]
use std::{
    fmt,
    path::Path,
    {ffi::OsStr, fs},
};

pub mod error;
mod pathfinder;
mod util;
pub mod validator;

pub use pathfinder::PathFinder;

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
