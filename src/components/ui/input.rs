use leptos::prelude::*;

// Input Component
#[component]
pub fn Input(
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] input_type: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let input_class = format!(
        "flex h-10 w-full rounded-2xl border border-gray-200 bg-white px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-950 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
        class.unwrap_or_default()
    );

    view! {
        <input
            class=input_class
            type=input_type.unwrap_or_else(|| "text".to_string())
            placeholder=placeholder.unwrap_or_default()
            value=value.unwrap_or_default()
            disabled=disabled.unwrap_or(false)
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    handler(event_target_value(&ev));
                }
            }
        />
    }
}
