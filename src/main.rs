use invomodo::routes::invoice_builder::InvoiceBuilder;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_meta::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::*;

use invomodo::auth::bindings::{self, AuthError, UserData};
use invomodo::auth::session::{self, SessionData, SessionState};

// Application state - all signals are Send + Sync safe
#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub user: RwSignal<Option<UserData>>,
    pub session_data: RwSignal<Option<SessionData>>,
    pub session_state: RwSignal<SessionState>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl AppState {
    // Session manager methods integrated into app state
    pub fn initialize_session(&self, server_data: bindings::ServerData) -> Result<(), AuthError> {
        session::initialize_session_data(server_data, &self.session_data, &self.session_state)
    }

    pub fn clear_session(&self) {
        session::clear_session_data();
        self.session_data.set(None);
        self.user.set(None);
        self.session_state.set(SessionState::Loading);
        self.error.set(None);
    }

    pub fn is_session_valid(&self) -> bool {
        session::is_session_valid(&self.session_state)
    }

    pub fn has_valid_subscription(&self) -> bool {
        session::has_valid_subscription(&self.session_data)
    }

    pub async fn refresh_session(&self, id_token: &str) -> Result<(), AuthError> {
        match bindings::fetch_server_data(id_token).await {
            Ok(server_data) => {
                self.initialize_session(server_data)?;
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to refresh session: {e:?}");
                self.session_state.set(SessionState::NetworkError);
                Err(e)
            }
        }
    }
}

// Main application component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize application state
    let app_state = AppState::default();

    // Load existing session on startup
    session::load_existing_session(&app_state.session_data, &app_state.session_state);

    provide_context(app_state.clone());

    // Initialize Firebase authentication
    Effect::new(move |_| {
        let app_state = app_state.clone();
        // spawn_local(async move {
        //     if let Err(e) = initialize_firebase_auth(app_state.clone()).await {
        //         log::error!("Firebase auth initialization failed: {e:?}");
        //         app_state
        //             .error
        //             .set(Some(format!("Authentication setup failed: {e}")));
        //     }
        //     app_state.loading.set(false);
        // });
    });

    view! {
        <Html attr:lang="en" />
        <Title text="Invomodo" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="description" content="Invomodo - Secure offline-first Invoice management app" />

        <Router>
            <div class="min-h-screen bg-gray-50">
                <AuthWrapper>
                    <Routes fallback=|| view! { "Page not found." }>
                        // Public routes outside Protected
                        // <Route path=path!("/builder") view=BuilderPage />
                        <Route path=path!("/blog") view=BlogPage />
                        <Route path=path!("/docs") view=DocsPage />
                        // <Route path=path!("/") view=HomePage />
                        <Route path=path!("/") view=InvoiceBuilder />
                        // Protected routes wrapped in <Protected>
                        <Route
                            path=path!("/dashboard")
                            view=|| {
                                view! {
                                    <Protected>
                                        <DashboardPage />
                                    </Protected>
                                }
                            }
                        />
                        <Route
                            path=path!("/profile")
                            view=|| {
                                view! {
                                    <Protected>
                                        <ProfilePage />
                                    </Protected>
                                }
                            }
                        />
                        <Route
                            path=path!("/settings")
                            view=|| {
                                view! {
                                    <Protected>
                                        <SettingsPage />
                                    </Protected>
                                }
                            }
                        />
                    </Routes>
                </AuthWrapper>
            </div>
        </Router>
    }
}

// Authentication wrapper component
#[component]
fn AuthWrapper(children: ChildrenFn) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let loading = app_state.loading;
    let error = app_state.error;

    view! {
        <Show
            when=move || loading.get()
            fallback={
                let children = children.clone();
                let error = error.clone();
                move || {
                    view! {
                        <Show
                            when=move || error.get().is_some()
                            fallback={
                                let children = children.clone();
                                move || children()
                            }
                        >
                            <ErrorScreen error=error.get().unwrap_or_default() />
                        </Show>
                    }
                }
            }
        >
            <LoadingScreen />
        </Show>
    }
}

// Protected route component
#[component]
pub fn Protected(children: ChildrenFn) -> impl IntoView {
    let children = children.clone();
    view! {
        <ShowWhenAuthenticated fallback=|| view! { <Login /> }>
            <LayoutProtectedArea children=children.clone() />
        </ShowWhenAuthenticated>
    }
}

// Layout for protected areas
#[component]
pub fn LayoutProtectedArea(children: ChildrenFn) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let user = app_state.user;
    let session_state = app_state.session_state;

    let children = children.clone();
    view! {
        <LayoutSelector user=user.get() session_state=session_state.get()>
            {children()}
        </LayoutSelector>
    }
}

// Show content only when authenticated with valid session and subscription
#[component(transparent)]
pub fn ShowWhenAuthenticated<C>(
    #[prop(into, optional)] fallback: ViewFn,

    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    C: IntoView + 'static,
{
    let app_state = expect_context::<AppState>();

    // Combine your auth, session, subscription checks into one reactive signal
    let is_authorized = Signal::derive(move || {
        app_state.user.get().is_some()
            && app_state.is_session_valid()
            && app_state.has_valid_subscription()
    });

    let children = children.into_inner();

    view! {
        <Show
            when=move || is_authorized.get()
            fallback={
                let fallback = fallback.clone();
                move || fallback.run()
            }
        >
            {children()}
        </Show>
    }
}

// Login component for unauthenticated users
#[component]
fn Login() -> impl IntoView {
    let app_state = expect_context::<AppState>();

    let handle_login = move |_| {
        let app_state = app_state.clone();
        spawn_local(async move {
            if let Err(e) = perform_login(app_state.clone()).await {
                app_state.error.set(Some(format!("Login failed: {}", e)));
            }
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Sign in to your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Access your secure dashboard and tools"
                    </p>
                </div>
                <div class="mt-8 space-y-6">
                    <button
                        on:click=handle_login
                        class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                    >
                        "🚀 Sign in with Google"
                    </button>
                    <div class="text-center">
                        <A href="/" attr:class="text-blue-600 hover:text-blue-500">
                            "← Back to public area"
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Layout selector component
#[component]
fn LayoutSelector(
    user: Option<UserData>,
    session_state: SessionState,
    children: Children,
) -> impl IntoView {
    let is_authenticated = user.is_some();
    let app_state = expect_context::<AppState>();
    let has_valid_subscription = app_state.has_valid_subscription();

    match (is_authenticated, session_state) {
        (true, SessionState::Valid | SessionState::Offline) if has_valid_subscription => {
            view! { <PrivateLayout user=user.unwrap()>{children()}</PrivateLayout> }.into_any()
        }
        (true, SessionState::SubscriptionExpired) => {
            view! { <SubscriptionExpiredLayout user=user.unwrap() /> }.into_any()
        }
        (true, SessionState::TamperingDetected) => view! { <SecurityWarningLayout /> }.into_any(),
        (true, SessionState::Expired) => {
            view! { <SessionExpiredLayout user=user.unwrap() /> }.into_any()
        }
        _ => view! { <PublicLayout>{children()}</PublicLayout> }.into_any(),
    }
}

// Public layout for unauthenticated users
#[component]
fn PublicLayout(children: Children) -> impl IntoView {
    view! {
        <div class="flex-1">
            <PublicHeader />
            <main class="grid grid-cols-1 lg:grid-cols-3 gap-6 p-6">{children()}</main>
            <PublicFooter />
        </div>
    }
}

// Private layout for authenticated users with valid subscription
#[component]
fn PrivateLayout(user: UserData, children: Children) -> impl IntoView {
    view! {
        <div class="private-layout flex">
            <Sidebar user=user.clone() />
            <div class="flex-1 flex flex-col">
                <PrivateHeader user=user.clone() />
                <main class="flex-1 p-6 bg-gray-50">{children()}</main>
            </div>
        </div>
    }
}

// Public header with login button
#[component]
fn PublicHeader() -> impl IntoView {
    let app_state = expect_context::<AppState>();

    let handle_login = move |_| {
        let app_state = app_state.clone();
        spawn_local(async move {
            if let Err(e) = perform_login(app_state.clone()).await {
                app_state.error.set(Some(format!("Login failed: {e}")));
            }
        });
    };

    view! {
        <header class="flex items-center justify-between whitespace-nowrap border-b border-[var(--border-color)] bg-[var(--background-secondary)] px-6 py-4">
            <div class="flex items-center gap-4">
                <div class="flex justify-between items-center py-4">
                    <div class="flex items-center">
                        <h1 class="text-xl font-semibold text-gray-900">"SecurePWA"</h1>
                    </div>
                    <nav class="flex items-center space-x-4">
                        <A href="/" attr:class="text-gray-600 hover:text-gray-900">
                            "Home"
                        </A>
                        <A href="builder" attr:class="text-gray-600 hover:text-gray-900">
                            "Builder"
                        </A>
                        <A href="/blog" attr:class="text-gray-600 hover:text-gray-900">
                            "Blog"
                        </A>
                        <A href="/docs" attr:class="text-gray-600 hover:text-gray-900">
                            "Docs"
                        </A>
                        <button
                            on:click=handle_login
                            class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition-colors"
                        >
                            "Sign In with Google"
                        </button>
                    </nav>
                </div>
            </div>
        </header>
    }
}

// Private header with user info and logout
#[component]
fn PrivateHeader(user: UserData) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let session_state = app_state.session_state;

    let handle_logout = move |_| {
        let app_state = app_state.clone();
        spawn_local(async move {
            if let Err(e) = perform_logout(app_state.clone()).await {
                log::error!("Logout failed: {:?}", e);
            }
        });
    };

    let session_status = move || match session_state.get() {
        SessionState::Valid => "🟢 Online",
        SessionState::Offline => "🟡 Offline",
        SessionState::Expired => "🔴 Session Expired",
        SessionState::TamperingDetected => "⚠️ Security Warning",
        SessionState::SubscriptionExpired => "❌ Subscription Expired",
        _ => "⏳ Loading...",
    };

    view! {
        <header class="bg-white shadow-sm border-b">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-4">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-xl font-semibold text-gray-900">"SecurePWA"</h1>
                        <span class="text-sm text-gray-500">{session_status}</span>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="flex items-center space-x-2">
                            <img
                                src=user.photo_url.clone()
                                alt="Profile"
                                class="w-8 h-8 rounded-full"
                            />
                            <span class="text-sm text-gray-700">{user.display_name.clone()}</span>
                        </div>
                        <button
                            on:click=handle_logout
                            class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md transition-colors"
                        >
                            "Logout"
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}

// Sidebar navigation for private layout
#[component]
fn Sidebar(user: UserData) -> impl IntoView {
    view! {
        <aside class="w-64 bg-gray-900 text-white flex flex-col">
            <div class="p-4 border-b border-gray-700">
                <div class="flex items-center space-x-3">
                    <img src=user.photo_url alt="Profile" class="w-10 h-10 rounded-full" />
                    <div>
                        <p class="font-medium">{user.display_name}</p>
                        <p class="text-sm text-gray-300">{user.email}</p>
                    </div>
                </div>
            </div>
            <nav class="flex-1 p-4">
                <ul class="space-y-2">
                    <li>
                        <A
                            href="/dashboard"
                            attr:class="block px-3 py-2 rounded-md hover:bg-gray-700 transition-colors"
                        >
                            // active_class="bg-gray-700"
                            "📊 Dashboard"
                        </A>
                    </li>
                    <li>
                        <A
                            href="/profile"
                            attr:class="block px-3 py-2 rounded-md hover:bg-gray-700 transition-colors"
                        >
                            // active_class="bg-gray-700"
                            "👤 Profile"
                        </A>
                    </li>
                    <li>
                        <A
                            href="/builder"
                            attr:class="block px-3 py-2 rounded-md hover:bg-gray-700 transition-colors"
                        >
                            // active_class="bg-gray-700"
                            "🔧 Builder"
                        </A>
                    </li>
                    <li>
                        <A
                            href="/settings"
                            attr:class="block px-3 py-2 rounded-md hover:bg-gray-700 transition-colors"
                        >
                            // active_class="bg-gray-700"
                            "⚙️ Settings"
                        </A>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}

// Loading screen component
#[component]
fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                <p class="text-gray-600">"Loading application..."</p>
            </div>
        </div>
    }
}

// Error screen component
#[component]
fn ErrorScreen(error: String) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center max-w-md">
                <div class="text-red-600 text-6xl mb-4">"⚠️"</div>
                <h1 class="text-2xl font-bold text-gray-900 mb-4">"Something went wrong"</h1>
                <p class="text-gray-600 mb-6">{error}</p>
                <button
                    on:click=|_| {
                        let _ = web_sys::window().unwrap().location().reload();
                    }
                    class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
                >
                    "Reload Page"
                </button>
            </div>
        </div>
    }
}

// Subscription expired layout
#[component]
fn SubscriptionExpiredLayout(user: UserData) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center max-w-md">
                <div class="text-orange-600 text-6xl mb-4">"⏰"</div>
                <h1 class="text-2xl font-bold text-gray-900 mb-4">"Subscription Expired"</h1>
                <p class="text-gray-600 mb-6">
                    "Hi " {user.display_name}
                    ", your subscription has expired. Please renew to continue using the app."
                </p>
                <button class="bg-green-600 text-white px-6 py-3 rounded-md hover:bg-green-700 mr-3">
                    "Renew Subscription"
                </button>
                <A
                    href="/"
                    attr:class="bg-gray-600 text-white px-6 py-3 rounded-md hover:bg-gray-700"
                >
                    "Go to Public Area"
                </A>
            </div>
        </div>
    }
}

// Security warning layout
#[component]
fn SecurityWarningLayout() -> impl IntoView {
    let app_state = expect_context::<AppState>();

    let handle_logout = move |_| {
        let app_state = app_state.clone();
        spawn_local(async move {
            let _ = perform_logout(app_state).await;
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-red-50">
            <div class="text-center max-w-md">
                <div class="text-red-600 text-6xl mb-4">"🚨"</div>
                <h1 class="text-2xl font-bold text-red-900 mb-4">"Security Warning"</h1>
                <p class="text-red-700 mb-6">
                    "We detected potential tampering with your session. For your security, please sign in again."
                </p>
                <button
                    on:click=handle_logout
                    class="bg-red-600 text-white px-6 py-3 rounded-md hover:bg-red-700"
                >
                    "Sign Out and Restart"
                </button>
            </div>
        </div>
    }
}

// Session expired layout
#[component]
fn SessionExpiredLayout(user: UserData) -> impl IntoView {
    let app_state = expect_context::<AppState>();

    let handle_refresh = move |_| {
        let app_state = app_state.clone();
        let user = user.clone();
        spawn_local(async move {
            if let Some(token) = user.id_token {
                let _ = app_state.refresh_session(&token).await;
            }
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center max-w-md">
                <div class="text-yellow-600 text-6xl mb-4">"⏱️"</div>
                <h1 class="text-2xl font-bold text-gray-900 mb-4">"Session Expired"</h1>
                <p class="text-gray-600 mb-6">
                    "Your offline session has expired. Please connect to the internet to refresh your session."
                </p>
                <button
                    on:click=handle_refresh
                    class="bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700 mr-3"
                >
                    "Refresh Session"
                </button>
                <A
                    href="/"
                    attr:class="bg-gray-600 text-white px-6 py-3 rounded-md hover:bg-gray-700"
                >
                    "Go to Public Area"
                </A>
            </div>
        </div>
    }
}

// Public footer
#[component]
fn PublicFooter() -> impl IntoView {
    view! {
        <footer class="bg-gray-900 text-white py-8">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <p class="text-sm text-gray-400">
                        "© 2025 SecurePWA. Built with Leptos and Firebase."
                    </p>
                </div>
            </div>
        </footer>
    }
}

// Page components
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-12">
            <div class="text-center">
                <h1 class="text-4xl font-bold text-gray-900 mb-6">"Welcome to SecurePWA"</h1>
                <p class="text-xl text-gray-600 mb-8">
                    "A secure, offline-first progressive web app with Firebase authentication and subscription management."
                </p>
                <div class="grid md:grid-cols-3 gap-6 mt-12">
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-3">"🔒 Secure"</h3>
                        <p class="text-gray-600">
                            "Advanced security features with tamper detection and secure session management."
                        </p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-3">"📱 Offline-First"</h3>
                        <p class="text-gray-600">
                            "Work seamlessly offline with configurable sync windows and data persistence."
                        </p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-3">"⚡ Fast"</h3>
                        <p class="text-gray-600">
                            "Built with Rust and Leptos for optimal performance and reactivity."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BlogPage() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-6">"Blog"</h1>
            <div class="space-y-8">
                <article class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-3">
                        "Building Secure PWAs with Rust and Leptos"
                    </h2>
                    <p class="text-gray-500 text-sm mb-3">"Published on January 15, 2025"</p>
                    <p class="text-gray-700">
                        "Learn how to build secure, offline-first progressive web applications using modern Rust frameworks..."
                    </p>
                </article>
                <article class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-3">
                        "Firebase Authentication Best Practices"
                    </h2>
                    <p class="text-gray-500 text-sm mb-3">"Published on January 10, 2025"</p>
                    <p class="text-gray-700">
                        "Implementing secure authentication flows with Firebase and protecting against common vulnerabilities..."
                    </p>
                </article>
            </div>
        </div>
    }
}

#[component]
fn DocsPage() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto px-4 py-8">
            <div class="flex">
                <aside class="w-64 pr-8">
                    <nav class="space-y-2">
                        <a href="#getting-started" class="block text-blue-600 hover:underline">
                            "Getting Started"
                        </a>
                        <a href="#authentication" class="block text-blue-600 hover:underline">
                            "Authentication"
                        </a>
                        <a href="#offline-mode" class="block text-blue-600 hover:underline">
                            "Offline Mode"
                        </a>
                        <a href="#security" class="block text-blue-600 hover:underline">
                            "Security"
                        </a>
                    </nav>
                </aside>
                <main class="flex-1">
                    <h1 class="text-3xl font-bold text-gray-900 mb-6">"Documentation"</h1>
                    <div class="prose max-w-none">
                        <section id="getting-started" class="mb-8">
                            <h2 class="text-2xl font-semibold mb-4">"Getting Started"</h2>
                            <p class="text-gray-700 mb-4">
                                "This application demonstrates a secure, offline-first PWA built with Leptos and Firebase authentication."
                            </p>
                        </section>
                        <section id="authentication" class="mb-8">
                            <h2 class="text-2xl font-semibold mb-4">"Authentication"</h2>
                            <p class="text-gray-700 mb-4">
                                "Authentication is handled through Firebase with Google Sign-In. The app supports both online and offline authentication states."
                            </p>
                        </section>
                        <section id="security" class="mb-8">
                            <h2 class="text-2xl font-semibold mb-4">"Security"</h2>
                            <p class="text-gray-700 mb-4">
                                "Advanced security measures include tamper detection, secure session management, and protection against clock manipulation attacks."
                            </p>
                        </section>
                    </div>
                </main>
            </div>
        </div>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let session_state = app_state.session_state;
    let user = app_state.user.get().unwrap();

    view! {
        <div class="max-w-6xl mx-auto">
            <h1 class="text-3xl font-bold text-gray-900 mb-6">"Dashboard"</h1>
            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold mb-2">"Session Status"</h3>
                    <p class="text-2xl font-bold text-green-600">
                        {move || match session_state.get() {
                            SessionState::Valid => "Active",
                            SessionState::Offline => "Offline",
                            _ => "Unknown",
                        }}
                    </p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold mb-2">"User"</h3>
                    <p class="text-sm text-gray-600">{user.display_name}</p>
                    <p class="text-sm text-gray-500">{user.email}</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold mb-2">"Subscription"</h3>
                    <p class="text-2xl font-bold text-blue-600">"Active"</p>
                </div>
            </div>
            <div class="bg-white rounded-lg shadow p-6">
                <h2 class="text-xl font-semibold mb-4">"Recent Activity"</h2>
                <div class="space-y-3">
                    <div class="flex justify-between items-center py-2 border-b">
                        <span>"Logged in"</span>
                        <span class="text-sm text-gray-500">"Just now"</span>
                    </div>
                    <div class="flex justify-between items-center py-2 border-b">
                        <span>"Session initialized"</span>
                        <span class="text-sm text-gray-500">"Just now"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProfilePage() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let user = app_state.user.get().unwrap();

    view! {
        <div class="max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold text-gray-900 mb-6">"Profile"</h1>
            <div class="bg-white rounded-lg shadow p-6">
                <div class="flex items-center space-x-6 mb-6">
                    <img
                        src=user.photo_url.clone()
                        alt="Profile Picture"
                        class="w-20 h-20 rounded-full"
                    />
                    <div>
                        <h2 class="text-xl font-semibold">{user.display_name.clone()}</h2>
                        <p class="text-gray-600">{user.email.clone()}</p>
                        <p class="text-sm text-gray-500">"UID: " {user.uid.clone()}</p>
                    </div>
                </div>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            "Display Name"
                        </label>
                        <input
                            type="text"
                            value=user.display_name
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            readonly
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input
                            type="email"
                            value=user.email
                            class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50"
                            readonly
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    view! {
        <div class="max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold text-gray-900 mb-6">"Settings"</h1>
            <div class="space-y-6">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-lg font-semibold mb-4">"Privacy Settings"</h2>
                    <div class="space-y-3">
                        <label class="flex items-center">
                            <input type="checkbox" class="mr-3" checked />
                            "Allow offline data storage"
                        </label>
                        <label class="flex items-center">
                            <input type="checkbox" class="mr-3" checked />
                            "Enable security monitoring"
                        </label>
                        <label class="flex items-center">
                            <input type="checkbox" class="mr-3" />
                            "Share usage analytics"
                        </label>
                    </div>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-lg font-semibold mb-4">"Session Settings"</h2>
                    <div class="space-y-3">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">
                                "Offline Window (hours)"
                            </label>
                            <input
                                type="number"
                                value="12"
                                min="1"
                                max="72"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                            <p class="text-xs text-gray-500 mt-1">
                                "How long you can use the app offline after last sync"
                            </p>
                        </div>
                    </div>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-lg font-semibold mb-4 text-red-700">"Danger Zone"</h2>
                    <button class="bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700 transition-colors">
                        "Clear All Data"
                    </button>
                    <p class="text-xs text-gray-500 mt-1">
                        "This will remove all locally stored data and log you out"
                    </p>
                </div>
            </div>
        </div>
    }
}

// Authentication helper functions

async fn initialize_firebase_auth(app_state: AppState) -> Result<(), AuthError> {
    use crate::bindings::initialize_auth;

    // Set up authentication state callback
    let user_signal = app_state.user;
    let error_signal = app_state.error;

    let auth_callback = {
        let app_state = app_state.clone();
        move |user_data: Option<UserData>| {
            let user_signal = user_signal.clone();
            let app_state = app_state.clone();
            let error_signal = error_signal.clone();

            spawn_local(async move {
                match user_data {
                    Some(user) => {
                        // User signed in - fetch server data and initialize session
                        if let Some(id_token) = &user.id_token {
                            match app_state.refresh_session(id_token).await {
                                Ok(_) => {
                                    user_signal.set(Some(user));
                                    error_signal.set(None);
                                }
                                Err(e) => {
                                    log::error!("Failed to initialize session: {e:?}");
                                    error_signal
                                        .set(Some(format!("Session initialization failed: {e}")));
                                }
                            }
                        } else {
                            user_signal.set(Some(user));
                        }
                    }
                    None => {
                        // User signed out - clear session
                        app_state.clear_session();
                        error_signal.set(None);
                    }
                }
            });
        }
    };

    // Initialize Firebase auth
    let current_user = initialize_auth(auth_callback)?;

    // Set initial user if already authenticated
    if let Some(user) = current_user {
        app_state.user.set(Some(user));
    }

    Ok(())
}

async fn perform_login(app_state: AppState) -> Result<(), AuthError> {
    use crate::bindings::sign_in_with_google;

    app_state.loading.set(true);
    app_state.error.set(None);

    let result = sign_in_with_google().await;

    match result {
        Ok(user_data) => {
            // Initialize session with server data
            if let Some(id_token) = &user_data.id_token {
                if let Err(e) = app_state.refresh_session(id_token).await {
                    app_state
                        .error
                        .set(Some(format!("Session setup failed: {e}")));
                    return Err(e);
                }
            }

            app_state.user.set(Some(user_data));
            Ok(())
        }
        Err(e) => {
            app_state.error.set(Some(format!("Login failed: {e}")));
            Err(e)
        }
    }
}

async fn perform_logout(app_state: AppState) -> Result<(), AuthError> {
    use crate::bindings::sign_out_user;

    app_state.loading.set(true);

    let result = sign_out_user().await;

    // Clear session regardless of sign-out result
    app_state.clear_session();
    app_state.loading.set(false);

    result
}

// Main function to start the application
fn main() {
    // Set up console error panic hook for better debugging
    console_error_panic_hook::set_once();

    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Starting Invomodo...");

    // Mount the application
    mount_to_body(|| {
        view! { <App /> }
    });
}
