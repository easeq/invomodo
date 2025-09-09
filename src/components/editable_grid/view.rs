use super::{DeviceType, FormData, TableRow, use_responsive};
use leptos::prelude::*;

/// Generic headless line items component with responsive support
#[component]
pub fn HeadlessLineItems<T, F, V>(
    /// Form render function
    form_renderer: F,
    /// Initial data
    #[prop(default = vec![])]
    initial_data: Vec<T>,
    /// Custom empty state message
    #[prop(default = "No items yet. Add one using the form above.".to_string())]
    empty_message: String,
) -> impl IntoView
where
    T: FormData + TableRow + Clone + PartialEq + Send + Sync + 'static,
    F: Fn(T::FormProps, Callback<T::FormProps>) -> V + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let (items, set_items) = signal(initial_data);
    let (current_form, set_current_form) = signal(T::FormProps::default());
    let (editing_index, set_editing_index) = signal(None::<usize>);

    // Use responsive hook
    let (_breakpoint, device_type) = use_responsive();

    // Handle form submission
    let on_submit = Callback::new(move |form_data: T::FormProps| {
        let new_item = T::from_form_props(&form_data);

        match editing_index.get_untracked() {
            Some(index) => {
                // Update existing item
                set_items.update(|items| {
                    if let Some(item) = items.get_mut(index) {
                        *item = new_item;
                    }
                });
                set_editing_index.set(None);
            }
            None => {
                // Add new item
                set_items.update(|items| items.push(new_item));
            }
        }

        // Reset form
        set_current_form.set(T::FormProps::default());
    });

    // Handle edit action
    let on_edit = Callback::new(move |index: usize| {
        let items_val = items.get_untracked();
        if let Some(item) = items_val.get(index) {
            set_current_form.set(item.to_form_props());
            set_editing_index.set(Some(index));
        }
    });

    // Handle delete action
    let on_delete = Callback::new(move |index: usize| {
        set_items.update(|items| {
            items.remove(index);
        });
        // Reset form if we were editing the deleted item
        if editing_index.get_untracked() == Some(index) {
            set_current_form.set(T::FormProps::default());
            set_editing_index.set(None);
        }
    });

    // Handle cancel edit
    let on_cancel = Callback::new(move |_| {
        set_current_form.set(T::FormProps::default());
        set_editing_index.set(None);
    });

    view! {
        <div class="space-y-6">
            // Form section
            <div class="bg-white p-4 md:p-6 rounded-lg shadow-sm border">
                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-4 space-y-2 sm:space-y-0">
                    <h2 class="text-lg font-semibold text-gray-900">
                        {move || {
                            if editing_index.get().is_some() { "Edit Item" } else { "Add New Item" }
                        }}
                    </h2>
                    <Show when=move || editing_index.get().is_some()>
                        <button
                            type="button"
                            class="px-3 py-1 text-sm text-gray-600 hover:text-gray-800 border border-gray-300 rounded-md"
                            on:click=move |_| on_cancel.run(())
                        >
                            "Cancel"
                        </button>
                    </Show>
                </div>
                {move || form_renderer.clone()(current_form.get(), on_submit)}
            </div>

            // Responsive content section
            <div class="bg-white rounded-lg shadow-sm border overflow-hidden">
                {move || {
                    let empty_message = empty_message.clone();
                    match device_type.get() {
                        DeviceType::Desktop => {
                            view! {
                                <div class="overflow-x-auto">
                                    <table class="min-w-full divide-y divide-gray-200">
                                        <thead class="bg-gray-50">{T::render_header()}</thead>
                                        <tbody class="bg-white divide-y divide-gray-200">
                                            <For
                                                each=move || items.get().into_iter().enumerate()
                                                key=|(i, _)| *i
                                                children=move |(index, item)| {
                                                    item.render_row(index, on_edit, on_delete)
                                                }
                                            />
                                            <Show when=move || items.get().is_empty()>
                                                <tr>
                                                    <td
                                                        colspan="100%"
                                                        class="px-6 py-12 text-center text-gray-500"
                                                    >
                                                        {empty_message.clone()}
                                                    </td>
                                                </tr>
                                            </Show>
                                        </tbody>
                                    </table>
                                </div>
                            }
                                .into_any()
                        }
                        DeviceType::Mobile => {
                            view! {
                                <div class="p-4">
                                    <For
                                        each=move || items.get().into_iter().enumerate()
                                        key=|(index, _item)| *index
                                        children=move |(index, item)| {
                                            view! {
                                                <div class="mb-4 last:mb-0">
                                                    {item.render_card(index, on_edit, on_delete)}
                                                </div>
                                            }
                                        }
                                    />
                                    <Show when=move || items.get().is_empty()>
                                        <div class="text-center py-12 text-gray-500">
                                            <div class="mb-4">
                                                <svg
                                                    class="mx-auto h-12 w-12 text-gray-300"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke="currentColor"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                                                    />
                                                </svg>
                                            </div>
                                            <p class="text-sm">{empty_message.clone()}</p>
                                        </div>
                                    </Show>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </div>

        // // Debug info (remove in production)
        // <Show when=cfg!(debug_assertions)>
        // <div class="text-xs text-gray-400 text-center">
        // "Device: " {move || format!("{:?}", device_type.get())} " | Breakpoint: "
        // {move || format!("{:?}", breakpoint.get())}
        // </div>
        // </Show>
        </div>
    }
}
