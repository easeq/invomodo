use leptos::prelude::*;
use std::collections::HashMap;

/// Breakpoint enum for different screen sizes
#[derive(Clone, PartialEq, Debug)]
pub enum Breakpoint {
    Mobile,  // < 640px
    Tablet,  // >= 640px && < 1024px
    Desktop, // >= 1024px
}

/// Device type for layout optimization
#[derive(Clone, PartialEq, Debug)]
pub enum DeviceType {
    Mobile,
    Desktop,
}

impl DeviceType {
    pub fn is_mobile(&self) -> bool {
        matches!(self, DeviceType::Mobile)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, DeviceType::Desktop)
    }
}

/// State for each item in the grid
#[derive(Clone, Debug, PartialEq)]
pub struct ItemState<T> {
    pub data: T,
    pub index: usize,
}

impl<T> ItemState<T> {
    pub fn new(data: T, index: usize) -> Self {
        Self { data, index }
    }
}

/// Form state information
#[derive(Clone, Debug, PartialEq)]
pub struct FormState<P> {
    pub current_form: P,
    pub editing_index: Option<usize>,
    pub is_editing: bool,
}

impl<P> FormState<P> {
    pub fn new(current_form: P, editing_index: Option<usize>) -> Self {
        Self {
            current_form,
            editing_index,
            is_editing: editing_index.is_some(),
        }
    }
}

/// Grid actions available to consumers
pub struct GridActions<P: 'static> {
    pub submit_form: Callback<P>,
    pub edit_item: Callback<usize>,
    pub delete_item: Callback<usize>,
    pub cancel_edit: Callback<()>,
    pub update_form: Callback<P>,
    pub clear_form: Callback<()>,
}

/// Complete grid state returned by the hook
pub struct GridState<T, P: 'static> {
    pub items: Vec<ItemState<T>>,
    pub form_state: FormState<P>,
    pub device_type: DeviceType,
    pub breakpoint: Breakpoint,
    pub actions: GridActions<P>,
    pub is_empty: bool,
    pub item_count: usize,
}

/// Validation result for forms
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub field_errors: HashMap<String, String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: vec![],
            field_errors: HashMap::new(),
        }
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.is_valid = false;
        self.errors.push(error);
        self
    }

    pub fn with_field_error(mut self, field: String, error: String) -> Self {
        self.is_valid = false;
        self.field_errors.insert(field, error);
        self
    }

    pub fn has_errors(&self) -> bool {
        !self.is_valid
    }
}
