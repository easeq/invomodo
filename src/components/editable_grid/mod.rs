pub mod hooks;
pub mod types;
pub mod validation;

// Re-export commonly used items
pub use hooks::{use_editable_grid, use_form_field, use_responsive};
pub use types::{
    Breakpoint, DeviceType, FormState, GridActions, GridState, ItemState, ValidationResult,
};
pub use validation::FormValidation;

/// Generic trait for form data that can be rendered and edited
pub trait FormData: Clone + PartialEq + 'static {
    type FormProps: Default + std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static;

    fn default() -> Self;
    fn to_form_props(&self) -> Self::FormProps;
    fn from_form_props(props: &Self::FormProps) -> Self;
}

/// Generic trait for providing item metadata (no rendering)
pub trait ItemData: Clone + PartialEq + 'static {
    /// Get unique identifier for the item
    fn get_id(&self) -> String;

    /// Get display title for the item
    fn get_title(&self) -> String;

    /// Get optional subtitle for the item
    fn get_subtitle(&self) -> Option<String> {
        None
    }

    /// Get any additional metadata as key-value pairs
    fn get_metadata(&self) -> Vec<(String, String)> {
        vec![]
    }
}
