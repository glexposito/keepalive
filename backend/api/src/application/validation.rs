use std::collections::BTreeMap;

/// Field-level validation failures collected by a use case before it runs,
/// shaped to map directly onto an RFC 7807 problem's `errors` member.
#[derive(Debug, Default)]
pub struct ValidationErrors {
    fields: BTreeMap<String, Vec<String>>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.fields.entry(field.into()).or_default().push(message.into());
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn into_fields(self) -> BTreeMap<String, Vec<String>> {
        self.fields
    }
}

impl From<garde::Report> for ValidationErrors {
    fn from(report: garde::Report) -> Self {
        let mut errors = Self::new();
        for (path, error) in report.iter() {
            errors.add(path.to_string(), error.to_string());
        }
        errors
    }
}
