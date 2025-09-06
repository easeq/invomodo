use leptos::html::*;
use leptos::prelude::*;

// Select Component
#[component]
pub fn Select(
    #[prop()] value: RwSignal<String>, // reactive value
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + 'static>>,
    children: Children,
) -> impl IntoView {
    let select_class = format!(
        "flex h-10 items-center justify-between rounded-2xl border border-gray-200 bg-white px-3 py-2 text-sm ring-offset-background placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-gray-950 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
        class.unwrap_or_default()
    );

    view! {
        <select
            class=select_class
            // Bind the select's value to the signal
            prop:value=move || value.get()
            // Update the signal when selection changes
            on:change=move |ev| {
                let new_val = event_target_value(&ev);
                value.set(new_val.clone());
                if let Some(handler) = &on_change {
                    handler(new_val);
                }
            }
        >
            {children()}
        </select>
    }
}
