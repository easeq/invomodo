use leptos::prelude::*;

use crate::shared::types::UserProfile;

// Navigation data structures
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub id: String,
    pub label: String,
    pub icon: String, // Icon name or class
    pub route: String,
    pub children: Option<Vec<NavItem>>,
    pub badge: Option<String>, // For notification badges
}

#[component]
pub fn Sidebar(
    nav_items: ReadSignal<Vec<NavItem>>,
    #[prop(optional)] user_profile: Option<UserProfile>,
    is_collapsed: ReadSignal<bool>,
    is_mobile_open: ReadSignal<bool>,
    on_nav_click: Callback<String>,
    #[prop(optional)] on_profile_action: Option<Callback<String>>,
    #[prop(optional)] on_close_mobile: Option<Callback<()>>,
) -> impl IntoView {
    let active_item = RwSignal::new(String::from("dashboard"));

    view! {
        <>
            // Mobile overlay
            <div
                class=move || if is_mobile_open.get() { "fixed inset-0 z-50 bg-black/80 lg:hidden" } else { "hidden" }
                on:click=move |_| {
                    if let Some(callback) = on_close_mobile {
                        callback.run(());
                    }
                }
            ></div>

            // Sidebar
            <div class=move || format!(
                "fixed top-0 z-50 h-full bg-background border-r border-border transition-all duration-300 lg:relative lg:translate-x-0 {}",
                if is_mobile_open.get() {
                    "translate-x-0"
                } else if is_collapsed.get() {
                    "w-16 -translate-x-full lg:translate-x-0"
                } else {
                    "w-64 -translate-x-full lg:translate-x-0"
                }
            )>
                <div class="flex h-full flex-col">
                    // Header
                    <div class="flex h-14 items-center border-b px-3">
                        {move || if is_collapsed.get() {
                            view! {
                                <div class="flex h-8 w-8 items-center justify-center rounded-md bg-primary">
                                    <span class="text-xs font-bold text-primary-foreground">"IB"</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex items-center space-x-2">
                                    <div class="h-6 w-6 rounded bg-primary"></div>
                                    <span class="font-semibold">"InvoiceBuilder"</span>
                                </div>
                            }.into_any()
                        }}

                        // Mobile close button
                        <button
                            class="ml-auto lg:hidden inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
                            on:click=move |_| {
                                if let Some(callback) = on_close_mobile {
                                    callback.run(());
                                }
                            }
                        >
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    </div>

                    // Navigation
                    <div class="flex-1 overflow-y-auto py-4">
                        <nav class="px-3 space-y-1">
                            <For
                                each=move || nav_items.get()
                                key=|item| item.id.clone()
                                children=move |item| {
                                    let item_id = item.id.clone();
                                    let is_active = move || active_item.get() == item_id;

                                    view! {
                                        <button
                                            class=move || format!(
                                                "flex w-full items-center rounded-md px-3 py-2 text-sm font-medium transition-colors {}",
                                                if is_active() {
                                                    "bg-accent text-accent-foreground"
                                                } else {
                                                    "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                                                }
                                            )
                                            on:click={
                                                let item_id = item.id.clone();
                                                move |_| {
                                                    active_item.set(item_id.clone());
                                                    on_nav_click.run(item_id.clone());
                                                }
                                            }
                                        >
                                            <div class="flex items-center">
                                                // Icon placeholder - you would replace with actual icons
                                                <div class="mr-3 h-4 w-4 rounded bg-muted"></div>
                                                {move || if !is_collapsed.get() {
                                                    view! {
                                                        <span>{item.label.clone()}</span>
                                                        {item.badge.as_ref().map(|badge| view! {
                                                            <span class="ml-auto rounded-full bg-primary px-2 py-0.5 text-xs text-primary-foreground">
                                                                {badge.as_str()}
                                                            </span>
                                                        })}
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }}
                                            </div>
                                        </button>
                                    }
                                }
                            />
                        </nav>
                    </div>

                    // User Profile
                    {user_profile.map(|profile| view! {
                        <div class="border-t p-3">
                            {move || if is_collapsed.get() {
                                view! {
                                    <button class="flex w-full items-center justify-center rounded-md p-2 hover:bg-accent">
                                        <div class="h-6 w-6 rounded-full bg-primary flex items-center justify-center">
                                            <span class="text-xs font-medium text-primary-foreground">
                                                {profile.name.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                            </span>
                                        </div>
                                    </button>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex items-center space-x-3 rounded-md p-2 hover:bg-accent">
                                        <div class="h-8 w-8 rounded-full bg-primary flex items-center justify-center">
                                            <span class="text-sm font-medium text-primary-foreground">
                                                {profile.name.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                            </span>
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <p class="text-sm font-medium truncate">{profile.name.clone()}</p>
                                            <p class="text-xs text-muted-foreground truncate">{profile.email.clone()}</p>
                                        </div>
                                        <button class="rounded-md p-1 hover:bg-accent">
                                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                            </svg>
                                        </button>
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    })}
                </div>
            </div>
        </>
    }
}
