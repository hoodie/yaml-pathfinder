//! Yaml Utility functions.
//!
//! Here are some missing batteries form the `yaml-rust` crate.
//! The cool thing about this is the simple path like access to nested elements.
//! if the yaml looks like this:
//!
//! ```yaml
//! programmer:
//!   name: Hendrik
//!   looks: good
//!   languages:
//!     * rust
//!     * ruby
//!     * python
//! ```
//!
//! you can access "ruby" like this: `get_string("programmer/languages/1")`.
//! Leading `/` will not be regarded.

#![allow(dead_code)]

use std::fs;
use std::path::Path;

#[cfg(feature = "date_parsing")]
use chrono::prelude::*;

pub use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Wrapper that opens and parses a `.yml` file.
pub fn open(path: &Path) -> Result<Yaml, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(&path)?;
    parse(&file_content)
}

/// Ruby like API to yaml-rust.
pub fn parse(file_content: &str) -> Result<Yaml, Box<dyn std::error::Error>> {
    Ok(YamlLoader::load_from_str(&file_content)?
        .get(0)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Yaml::from_str("[]")))
}

/// Interprets `"25.12.2016"` as date.
#[cfg(feature = "date_parsing")]
pub fn parse_dmy_date(date_str: &str) -> Option<Date<Utc>> {
    let date = date_str
        .split('.')
        .map(|f| f.parse().unwrap_or(0))
        .collect::<Vec<i32>>();
    if date.len() >= 2 && date[0] > 0 && date[2] > 1900 {
        // XXX this neglects the old "01-05.12.2015" format
        Utc.ymd_opt(date[2], date[1] as u32, date[0] as u32)
            .single()
    } else {
        None
    }
}

/// Interprets `"24-25.12.2016"` as date.
///
/// Takes care of the old, deprecated, stupid, `dd-dd.mm.yyyy` format, what was I thinking?
/// This is not used in the current format.
#[cfg(feature = "date_parsing")]
pub fn parse_dmy_date_range(date_str: &str) -> Option<Date<Utc>> {
    let date = date_str
        .split('.')
        .map(|s| s.split('-').nth(0).unwrap_or("0"))
        .map(|f| f.parse().unwrap_or(0))
        .collect::<Vec<i32>>();
    if date[0] > 0 {
        return Some(Utc.ymd(date[2], date[1] as u32, date[0] as u32));
    }
    None
}
