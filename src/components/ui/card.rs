use leptos::html::*;
use leptos::prelude::*;

// Card Component
#[component]
pub fn Card(#[prop(into, optional)] class: Option<String>, children: Children) -> impl IntoView {
    let card_class = format!(
        "rounded-2xl border border-gray-200 bg-white text-gray-950 shadow-sm {}",
        class.unwrap_or_default()
    );

    view! {
        <div class=card_class>
            {children()}
        </div>
    }
}
