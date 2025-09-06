use leptos::prelude::*;

use crate::components::layout::top_nav::TopNav;
use crate::shared::types::{LayoutMode, UserProfile};
use crate::components::layout::sidebar::{NavItem, Sidebar};

#[component]
pub fn AppLayout(
    mode: LayoutMode,
    #[prop(optional)] nav_items: Option<ReadSignal<Vec<NavItem>>>,
    #[prop(optional)] user_profile: Option<UserProfile>,
    children: Children,
    #[prop(optional)] on_nav_click: Option<Callback<String>>,
    #[prop(optional)] on_login: Option<Callback<()>>,
    #[prop(optional)] on_signup: Option<Callback<()>>,
) -> impl IntoView {
    let sidebar_collapsed = RwSignal::new(false);
    let mobile_sidebar_open = RwSignal::new(false);

    let toggle_sidebar = move |_| {
        sidebar_collapsed.update(|collapsed| *collapsed = !*collapsed);
    };

    let toggle_mobile_sidebar = move |_| {
        mobile_sidebar_open.update(|open| *open = !*open);
    };

    let close_mobile_sidebar = move |_| {
        mobile_sidebar_open.set(false);
    };

    let noop = Callback::new(|_| {});
    match mode {
        LayoutMode::Public => {
            view! {
                <div class="min-h-screen bg-background">
                    <TopNav
                        is_authenticated=false
                        on_login=on_login.unwrap_or_else(|| noop.clone())
                        on_signup=on_signup.unwrap_or_else(|| noop.clone())
                    />
                    <main class="container mx-auto px-2 py-8">
                        {children()}
                    </main>
                </div>
            }.into_any()
        }
        LayoutMode::Authenticated => {
            let nav_items = nav_items.unwrap_or_else(|| {
                RwSignal::new(vec![
                    NavItem {
                        id: "dashboard".to_string(),
                        label: "Dashboard".to_string(),
                        icon: "dashboard".to_string(),
                        route: "/dashboard".to_string(),
                        children: None,
                        badge: None,
                    },
                    NavItem {
                        id: "clients".to_string(),
                        label: "Clients".to_string(),
                        icon: "users".to_string(),
                        route: "/clients".to_string(),
                        children: None,
                        badge: Some("12".to_string()),
                    },
                    NavItem {
                        id: "invoices".to_string(),
                        label: "Invoices".to_string(),
                        icon: "file-text".to_string(),
                        route: "/invoices".to_string(),
                        children: None,
                        badge: None,
                    },
                    NavItem {
                        id: "templates".to_string(),
                        label: "Templates".to_string(),
                        icon: "layout".to_string(),
                        route: "/templates".to_string(),
                        children: None,
                        badge: None,
                    },
                    NavItem {
                        id: "backups".to_string(),
                        label: "Backups".to_string(),
                        icon: "archive".to_string(),
                        route: "/backups".to_string(),
                        children: None,
                        badge: None,
                    },
                    NavItem {
                        id: "settings".to_string(),
                        label: "Settings".to_string(),
                        icon: "settings".to_string(),
                        route: "/settings".to_string(),
                        children: None,
                        badge: None,
                    },
                ]).read_only()
            });

            let default_profile = UserProfile {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                avatar_url: None,
            };

            view! {
                <div class="min-h-screen bg-background">
                    <TopNav
                        is_authenticated=true
                        on_menu_toggle=Callback::new(toggle_mobile_sidebar)
                    />
                    
                    <div class="flex h-[calc(100vh-3.5rem)]">
                        <Sidebar
                            nav_items=nav_items
                            user_profile=user_profile.unwrap_or(default_profile)
                            is_collapsed=sidebar_collapsed.read_only()
                            is_mobile_open=mobile_sidebar_open.read_only()
                            on_nav_click=on_nav_click.unwrap_or_else(|| Callback::new(|_| {}))
                            on_close_mobile=Callback::new(close_mobile_sidebar)
                        />
                        
                        <main class=move || format!(
                            "flex-1 overflow-y-auto transition-all duration-300 {}",
                            if sidebar_collapsed.get() { "lg:ml-16" } else { "lg:ml-64" }
                        )>
                            <div class="container mx-auto p-6">
                                // Collapse button for desktop
                                <div class="hidden lg:block mb-4">
                                    <button
                                        class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
                                        on:click=toggle_sidebar
                                    >
                                        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                        </svg>
                                    </button>
                                </div>
                                
                                {children()}
                            </div>
                        </main>
                    </div>
                </div>
            }.into_any()
        }
    }
}
