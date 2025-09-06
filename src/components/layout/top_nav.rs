use leptos::prelude::*;

#[component]
pub fn TopNav(
    #[prop(default = false)] is_authenticated: bool,
    #[prop(optional)] on_menu_toggle: Option<Callback<()>>,
    #[prop(optional)] on_login: Option<Callback<()>>,
    #[prop(optional)] on_signup: Option<Callback<()>>,
) -> impl IntoView {
    let is_mobile_menu_open = RwSignal::new(false);

    let toggle_mobile_menu = move |_| {
        is_mobile_menu_open.update(|open| *open = !*open);
    };

    view! {
        <header class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="container flex h-14 max-w-screen-2xl items-center px-4">
                // Logo
                <div class="mr-4 hidden md:flex">
                    <a class="mr-6 flex items-center space-x-2" href="/">
                        <div class="h-6 w-6 rounded bg-primary"></div>
                        <span class="hidden font-bold sm:inline-block">InvoiceBuilder</span>
                    </a>
                </div>

                // Mobile menu button (authenticated mode)
                {if is_authenticated {
                    view! {
                        <button
                            class="mr-2 md:hidden inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
                            on:click=move |_| {
                                if let Some(callback) = on_menu_toggle {
                                    callback.run(());
                                }
                            }
                        >
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}

                // Logo (mobile)
                <div class="flex md:hidden">
                    <a class="flex items-center space-x-2" href="/">
                        <div class="h-6 w-6 rounded bg-primary"></div>
                        <span class="font-bold">InvoiceBuilder</span>
                    </a>
                </div>

                // Desktop navigation (public mode)
                {if !is_authenticated {
                    view! {
                        <nav class="flex items-center space-x-6 text-sm font-medium hidden md:flex ml-6">
                            <a class="transition-colors hover:text-foreground/80 text-foreground/60" href="/docs">
                                "Docs"
                            </a>
                            <a class="transition-colors hover:text-foreground/80 text-foreground/60" href="/blog">
                                "Blog"
                            </a>
                            <a class="transition-colors hover:text-foreground/80 text-foreground" href="/builder">
                                "Builder"
                            </a>
                        </nav>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}

                <div class="flex flex-1 items-center justify-end space-x-4">
                    // Public mode actions
                    {if !is_authenticated {
                        view! {
                            <>
                                // Mobile menu button (public)
                                <button
                                    class="md:hidden inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
                                    on:click=toggle_mobile_menu
                                >
                                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                    </svg>
                                </button>

                                // Desktop auth buttons
                                <div class="hidden md:flex items-center space-x-2">
                                    <button
                                        class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                                        on:click=move |_| {
                                            if let Some(callback) = on_login {
                                                callback.run(());
                                            }
                                        }
                                    >
                                        "Log In"
                                    </button>
                                    <button
                                        class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-3"
                                        on:click=move |_| {
                                            if let Some(callback) = on_signup {
                                                callback.run(());
                                            }
                                        }
                                    >
                                        "Sign Up"
                                    </button>
                                </div>
                            </>
                        }.into_any()
                    } else {
                        view! {
                            // Authenticated mode: Search and user menu
                            <div class="flex items-center space-x-4">
                                <div class="relative hidden md:block">
                                    <svg class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                                    </svg>
                                    <input
                                        class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 pl-8 md:w-[300px] lg:w-[400px]"
                                        placeholder="Search..."
                                        type="search"
                                    />
                                </div>
                                // Notifications
                                <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9 relative">
                                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5-5-5h5V3h0z"></path>
                                    </svg>
                                    <span class="absolute -top-1 -right-1 h-2 w-2 rounded-full bg-red-500"></span>
                                </button>
                            </div>
                        }.into_any()
                    }}
                </div>
            </div>

            // Mobile menu (public mode)
            <div class=move || if is_mobile_menu_open.get() && !is_authenticated { "md:hidden border-t border-border" } else { "hidden" }>
                <div class="container px-4 py-4">
                    <nav class="flex flex-col space-y-4">
                        <a class="text-sm font-medium transition-colors hover:text-primary" href="/docs">"Docs"</a>
                        <a class="text-sm font-medium transition-colors hover:text-primary" href="/blog">"Blog"</a>
                        <a class="text-sm font-medium transition-colors hover:text-primary" href="/builder">"Builder"</a>
                        <div class="flex flex-col space-y-2 pt-2 border-t">
                            <button
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                                on:click=move |_| {
                                    if let Some(callback) = on_login {
                                        callback.run(());
                                    }
                                }
                            >
                                "Log In"
                            </button>
                            <button
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-3"
                                on:click=move |_| {
                                    if let Some(callback) = on_signup {
                                        callback.run(());
                                    }
                                }
                            >
                                "Sign Up"
                            </button>
                        </div>
                    </nav>
                </div>
            </div>
        </header>
    }
}
