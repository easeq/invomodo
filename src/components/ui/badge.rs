use leptos::html::*;
use leptos::prelude::*;

// Badge Component
#[component]
pub fn Badge(
    #[prop(into, optional)] variant: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "default".to_string());

    let badge_class = format!(
        "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-gray-950 focus:ring-offset-2 {}",
        match variant.as_str() {
            "secondary" => "bg-gray-100 text-gray-900 hover:bg-gray-200",
            "destructive" => "bg-red-100 text-red-900 hover:bg-red-200",
            "outline" => "border border-gray-200 text-gray-950",
            _ => "bg-gray-900 text-gray-50 hover:bg-gray-800",
        }
    );

    view! {
        <div class=format!("{} {}", badge_class, class.unwrap_or_default())>
            {children()}
        </div>
    }
}
