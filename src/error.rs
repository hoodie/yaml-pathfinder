#![allow(missing_docs)]

use thiserror::Error;

/// Result of validating part of a project.
///
/// We have to differentiate between incomplete data (missing values) and wrong data (invalid values).
/// Wrong data should always be a hard error - there is no reason to have invalid values in project files.
///
/// Missing data is not an error, it simply means that some information is not available yet.
/// An example is the field for the payment date: this field is missing until the invoice has been paid,
/// since the date is unknown until that point.
#[derive(Eq, PartialEq, Debug, Default)]
pub struct ValidationResult {
    /// hard error messages (invalid data)
    pub validation_errors: Vec<String>,

    /// soft error messages (incomplete data)
    pub missing_fields: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            validation_errors: Vec::new(),
            missing_fields: Vec::new(),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.validation_errors.is_empty() && self.missing_fields.is_empty()
    }

    pub fn validate_field<T>(&mut self, name: &str, val: FieldResult<T>) {
        if let Err(FieldError::Invalid(msg)) = val {
            self.validation_errors
                .push(format!("{:?} is invalid: {}", name, msg));
        }
    }

    pub fn require_option<T>(&mut self, name: &str, val: Option<T>) {
        if val.is_none() {
            self.missing_fields.push(name.to_string())
        }
    }

    pub fn require_field<T>(&mut self, name: &str, val: FieldResult<T>) {
        if val.is_err() {
            self.missing_fields.push(name.to_string())
        }

        if let Err(FieldError::Invalid(msg)) = val {
            self.validation_errors
                .push(format!("{:?} is invalid: {}", name, msg));
        }
    }

    pub fn and(mut self, next: ValidationResult) -> ValidationResult {
        self.missing_fields.extend(next.missing_fields);
        self.validation_errors.extend(next.validation_errors);
        self
    }

    pub fn unwrap(self) {
        let no_errors: Vec<String> = Vec::with_capacity(0);
        let no_missing: Vec<String> = Vec::with_capacity(0);
        assert_eq!(
            (self.validation_errors, self.missing_fields),
            (no_errors, no_missing)
        );
    }
}

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

pub trait FieldResultExt<T> {
    /// Tries an alternative only if the original is actually Missing.
    ///
    /// This makes sure we don't accidentally fall back to an old spec value if the original is invalid.
    fn if_missing_try<F: FnOnce() -> FieldResult<T>>(self, f: F) -> FieldResult<T>;
    fn invalid(&self) -> Option<&str>;
    fn is_invalid(&self) -> bool;
    fn is_missing(&self) -> bool;
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

    fn invalid(&self) -> Option<&str> {
        if let Err(FieldError::Invalid(msg)) = self {
            Some(msg)
        } else {
            None
        }
    }

    fn is_invalid(&self) -> bool {
        self.invalid().is_some()
    }
}
