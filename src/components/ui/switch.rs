use leptos::html::*;
use leptos::prelude::*;

// Switch Component
#[component]
pub fn Switch(
    #[prop(into)] checked: bool,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool) + 'static>>,
) -> impl IntoView {
    let switch_class = format!(
        "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-950 focus-visible:ring-offset-2 focus-visible:ring-offset-white disabled:cursor-not-allowed disabled:opacity-50 {} {}",
        if checked {
            "bg-gray-900"
        } else {
            "bg-gray-200"
        },
        class.unwrap_or_default()
    );

    let thumb_class = format!(
        "pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform {}",
        if checked {
            "translate-x-5"
        } else {
            "translate-x-0"
        }
    );

    view! {
        <button
            class=switch_class
            role="switch"
            aria-checked=checked
            on:click=move |_| {
                if let Some(handler) = &on_change {
                    handler(!checked);
                }
            }
        >
            <span class=thumb_class></span>
        </button>
    }
}
