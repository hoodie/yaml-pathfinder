use crate::{error::*, PathFinder};
use yaml_rust::Yaml;

pub trait Rule: Send + Sync + 'static {
    type Res: Invalidatable + Send + 'static;
    fn call(&self, data: &Yaml) -> Self::Res;
}

impl<F: Send + Sync + 'static, R> Rule for F
where
    F: Fn(&Yaml) -> R,
    R: Invalidatable + Send + 'static,
{
    type Res = R;

    fn call(&self, req: &Yaml) -> R {
        (self)(req)
    }
}

pub(crate) type DynRule = dyn (Fn(&Yaml) -> Box<dyn Invalidatable>);

fn box_rule(rule: impl Rule) -> Box<DynRule> {
    Box::new(move |cx| Box::new(rule.call(cx)))
}

#[derive(Default)]
pub struct Validator {
    rules: Vec<Box<DynRule>>,
    required: Vec<String>,
}

impl Validator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_rule(&mut self, rule: impl Rule) -> &mut Self {
        self.rules.push(box_rule(rule));
        self
    }

    pub fn require(&mut self, path: &str) -> &mut Self {
        self.required.push(path.into());
        self
    }

    pub fn fin(&mut self) -> Self {
        Self {
            rules: self.rules.drain(..).collect(),
            required: self.required.drain(..).collect(),
        }
    }

    pub fn validate(&self, data: &Yaml) -> ValidationResult {
        let missing_fields = self
            .required
            .iter()
            .filter(|path| data.get(path).is_some())
            .map(Into::into)
            .collect::<Vec<String>>();

        let validation_errors = self
            .rules
            .iter()
            .filter_map(|rule| rule(data).invalid().map(Into::into))
            .collect::<Vec<String>>();

        ValidationResult {
            validation_errors,
            missing_fields,
        }
    }
}

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
