use super::types::ValidationResult;

/// Trait for form validation
pub trait FormValidation {
    fn validate(&self) -> ValidationResult;
}

/// Common validation functions
pub mod validators {
    use super::ValidationResult;

    pub fn required(value: &str, field_name: &str) -> ValidationResult {
        if value.trim().is_empty() {
            ValidationResult::new()
                .with_field_error(field_name.to_string(), format!("{field_name} is required"))
        } else {
            ValidationResult::new()
        }
    }

    pub fn min_length(value: &str, min: usize, field_name: &str) -> ValidationResult {
        if value.len() < min {
            ValidationResult::new().with_field_error(
                field_name.to_string(),
                format!("{field_name} must be at least {min} characters"),
            )
        } else {
            ValidationResult::new()
        }
    }

    pub fn max_length(value: &str, max: usize, field_name: &str) -> ValidationResult {
        if value.len() > max {
            ValidationResult::new().with_field_error(
                field_name.to_string(),
                format!("{field_name} must be no more than {max} characters"),
            )
        } else {
            ValidationResult::new()
        }
    }

    pub fn email(value: &str, field_name: &str) -> ValidationResult {
        let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if !email_regex.is_match(value) {
            ValidationResult::new().with_field_error(
                field_name.to_string(),
                format!("{field_name} must be a valid email address"),
            )
        } else {
            ValidationResult::new()
        }
    }

    pub fn combine_results(results: Vec<ValidationResult>) -> ValidationResult {
        let mut combined = ValidationResult::new();

        for result in results {
            if !result.is_valid {
                combined.is_valid = false;
                combined.errors.extend(result.errors);
                combined.field_errors.extend(result.field_errors);
            }
        }

        combined
    }
}
