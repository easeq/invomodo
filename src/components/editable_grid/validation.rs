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
        let result = ValidationResult::new();

        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} must contain a single '@' symbol"),
            );
        }

        let local = parts[0];
        let domain = parts[1];

        // Local part rules
        if local.is_empty() {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} local part must not be empty"),
            );
        }

        if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
            return result.with_field_error(
            field_name.to_string(),
            format!("{field_name} local part must not start/end with dot or contain consecutive dots"),
        );
        }

        if !local
            .chars()
            .all(|c| c.is_alphanumeric() || "!#$%&'*+/=?^_`{|}~.-".contains(c) || !c.is_ascii())
        {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} local part contains invalid characters"),
            );
        }

        // Domain rules
        if domain.is_empty() {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} domain must not be empty"),
            );
        }

        if domain.starts_with('.') || domain.ends_with('.') {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} domain must not start or end with a dot"),
            );
        }

        let labels: Vec<&str> = domain.split('.').collect();
        if labels.len() < 2 || labels.iter().any(|l| l.is_empty()) {
            return result.with_field_error(
                field_name.to_string(),
                format!("{field_name} domain must contain at least one '.' and valid labels"),
            );
        }

        for label in labels {
            if !label
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || !c.is_ascii())
            {
                return result.with_field_error(
                    field_name.to_string(),
                    format!("{field_name} domain label '{label}' contains invalid characters"),
                );
            }

            if let Some(first) = label.chars().next() {
                if !first.is_alphanumeric() && first.is_ascii() {
                    return result.with_field_error(
                        field_name.to_string(),
                        format!(
                            "{field_name} domain label '{label}' must start with a letter or digit"
                        ),
                    );
                }
            }

            if let Some(last) = label.chars().last() {
                if !last.is_alphanumeric() && last.is_ascii() {
                    return result.with_field_error(
                        field_name.to_string(),
                        format!(
                            "{field_name} domain label '{label}' must end with a letter or digit"
                        ),
                    );
                }
            }
        }

        result
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
