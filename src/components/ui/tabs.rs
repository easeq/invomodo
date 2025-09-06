use leptos::html::*;
use leptos::prelude::*;
use std::sync::Arc;

// Tabs Components
#[component]
pub fn Tabs(
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + Send + Sync + 'static>>,
    children: Children,
) -> impl IntoView {
    provide_context(TabsContext {
        value,
        on_change: on_change.map(|cb| Arc::from(cb)), // convert Box to Arc
    });

    view! {
        <div class="space-y-4">
            {children()}
        </div>
    }
}

#[derive(Clone)]
struct TabsContext {
    value: RwSignal<String>,
    on_change: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}

#[component]
pub fn TabsList(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let list_class = format!(
        "inline-flex h-10 items-center justify-center rounded-2xl bg-gray-100 p-1 text-gray-500 {}",
        class.unwrap_or_default()
    );

    view! {
        <div class=list_class>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let ctx = use_context::<TabsContext>().expect("TabsTrigger must be used within Tabs");

    let trigger_class = {
        let active_tab = ctx.value.clone();
        let value = value.clone();
        move || {
            let is_active = active_tab.get() == value;
            format!(
                "inline-flex items-center justify-center whitespace-nowrap rounded-xl px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}",
                if is_active {
                    "bg-white text-gray-950 shadow-sm"
                } else {
                    "text-gray-500 hover:text-gray-900"
                }
            )
        }
    };

    let value_clone = value.clone();
    view! {
        <button
            class=trigger_class
            on:click=move |_| {
                if let Some(handler) = &ctx.on_change {
                    handler(value_clone.clone());
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let ctx = use_context::<TabsContext>().expect("TabsContent must be used within Tabs");

    let is_active = Memo::new(move |_| ctx.value.get() == value);

    view! {
        <div
            class="mt-4 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-950 focus-visible:ring-offset-2"
            style=move || {
                if is_active.get() {
                    "display: block;"
                } else {
                    "display: none;"
                }
            }
        >
            {children()}
        </div>
    }
}
