use leptos::prelude::*;

use crate::components::invoice::InvoiceBuilder as InvoiceBuilderView;

#[component]
pub fn InvoiceBuilder() -> impl IntoView {
    view! { <InvoiceBuilderView /> }
}
