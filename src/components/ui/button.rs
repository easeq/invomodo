use leptos::html::*;
use leptos::prelude::*;

// Button Component
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Option<String>,
    #[prop(into, optional)] size: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "default".to_string());
    let size = size.unwrap_or_else(|| "default".to_string());

    let button_class = format!(
        "inline-flex items-center justify-center rounded-2xl font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {} {} {}",
        match variant.as_str() {
            "destructive" => "bg-red-500 text-white hover:bg-red-600",
            "outline" => "border border-gray-200 bg-white hover:bg-gray-50 hover:text-gray-900",
            "secondary" => "bg-gray-100 text-gray-900 hover:bg-gray-200",
            "ghost" => "hover:bg-gray-100 hover:text-gray-900",
            "link" => "text-blue-600 underline-offset-4 hover:underline",
            _ => "bg-gray-900 text-white hover:bg-gray-800",
        },
        match size.as_str() {
            "sm" => "h-9 px-3",
            "lg" => "h-11 px-8",
            _ => "h-10 px-4 py-2",
        },
        class.unwrap_or_default()
    );

    view! {
        <button
            class=button_class
            disabled=disabled.unwrap_or(false)
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}
