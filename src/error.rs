#![allow(missing_docs)]

use thiserror::Error;

#[derive(Error, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum FieldError {
    #[error("The expected field is missing")]
    Missing,

    #[error("The field has an invalid value or type")]
    Invalid(String),
}

impl FieldError {
    pub fn invalid(e: &str) -> FieldError {
        FieldError::Invalid(e.to_owned())
    }
}

pub type FieldResult<T> = Result<T, FieldError>;

pub trait Invalidatable {
    fn invalid(&self) -> Option<&str>;
}

pub trait FieldResultExt<T> {
    /// Tries an alternative only if the original is actually Missing.
    ///
    /// This makes sure we don't accidentally fall back to an old spec value if the original is invalid.
    fn if_missing_try<F: FnOnce() -> FieldResult<T>>(self, f: F) -> FieldResult<T>;
    fn is_invalid(&self) -> bool;
    fn is_missing(&self) -> bool;
}

impl<T> Invalidatable for FieldResult<T> {
    fn invalid(&self) -> Option<&str> {
        if let Err(FieldError::Invalid(msg)) = self {
            Some(msg)
        } else {
            None
        }
    }
}

impl<T> FieldResultExt<T> for FieldResult<T> {
    fn if_missing_try<F: FnOnce() -> FieldResult<T>>(self, f: F) -> FieldResult<T> {
        if let Err(FieldError::Missing) = self {
            f()
        } else {
            self
        }
    }

    fn is_missing(&self) -> bool {
        if let Err(FieldError::Missing) = self {
            true
        } else {
            false
        }
    }

    fn is_invalid(&self) -> bool {
        self.invalid().is_some()
    }
}
