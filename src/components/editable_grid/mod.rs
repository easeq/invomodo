use leptos::prelude::*;

mod hooks;
mod view;

pub use hooks::*;
pub use view::*;

/// Generic trait for form data that can be rendered and edited
pub trait FormData: Clone + PartialEq + 'static {
    type FormProps: Default + Clone + PartialEq + Send + Sync + 'static;

    fn default() -> Self;
    fn to_form_props(&self) -> Self::FormProps;
    fn from_form_props(props: &Self::FormProps) -> Self;
}

/// Generic trait for table row rendering with mobile card support
pub trait TableRow: Clone + PartialEq + 'static {
    /// Render as table row (desktop)
    fn render_row(
        self,
        index: usize,
        on_edit: Callback<usize>,
        on_delete: Callback<usize>,
    ) -> impl IntoView;

    /// Render table header (desktop)
    fn render_header() -> impl IntoView;

    /// Render as card (mobile) - default implementation uses table row
    fn render_card(
        self,
        index: usize,
        on_edit: Callback<usize>,
        on_delete: Callback<usize>,
    ) -> impl IntoView {
        self.render_mobile_card(index, on_edit, on_delete)
    }

    /// Default mobile card implementation - can be overridden
    fn render_mobile_card(
        self,
        index: usize,
        on_edit: Callback<usize>,
        on_delete: Callback<usize>,
    ) -> impl IntoView;

    /// Get card title for mobile view
    fn get_card_title(&self) -> String;

    /// Get card subtitle for mobile view (optional)
    fn get_card_subtitle(&self) -> Option<String> {
        None
    }
}
