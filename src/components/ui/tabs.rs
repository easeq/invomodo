use leptos::prelude::*;

// Tabs Components
#[derive(Clone)]
pub struct TabsContext {
    active: ReadSignal<String>,
    set_active: WriteSignal<String>,
}

#[component]
pub fn TabsProvider(default: String, children: Children) -> impl IntoView {
    let (active, set_active) = signal(default);

    provide_context(TabsContext { active, set_active });

    view! { {children()} }
}

pub fn use_tab(name: String) -> (Memo<bool>, impl Fn()) {
    let ctx = use_context::<TabsContext>().expect("use_tab must be called within <TabsProvider>");
    let is_selected = Memo::new({
        let name = name.clone();
        move |_| ctx.active.get() == name
    });
    let select = {
        let ctx = ctx.clone();
        move || ctx.set_active.set(name.clone())
    };
    (is_selected, select)
}

pub fn use_tab_panel(name: String) -> Memo<bool> {
    let ctx =
        use_context::<TabsContext>().expect("use_tab_panel must be used within <TabsProvider>");
    Memo::new(move |_| ctx.active.get() == name)
}

#[component]
pub fn Tab<F, R>(name: String, render: F) -> impl IntoView
where
    F: Fn(Signal<bool>, Box<dyn Fn()>) -> R + 'static,
    R: IntoView + 'static,
{
    let (is_selected, select) = use_tab(name.clone());
    view! { {render(is_selected.into(), Box::new(select))} }
}

#[component]
pub fn TabPanel(name: String, children: ChildrenFn) -> impl IntoView {
    let visible = use_tab_panel(name.clone());
    view! {
        {
            let children = children.clone();
            move || if visible.get() { Some(children()) } else { None }.into_any()
        }
    }
}
