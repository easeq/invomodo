use crate::components::invoice::FieldItemValue;
use crate::components::invoice::{ContainerCallbacks, ContainerRenderData, ContainerRenderer};
use leptos::prelude::*;
use std::collections::HashMap;

pub fn create_container_renderer(
    title: Option<String>,
    is_collapsed: RwSignal<bool>,
    _form_values: RwSignal<HashMap<String, FieldItemValue>>,
    _on_submit: Option<Callback<HashMap<String, FieldItemValue>>>,
) -> ContainerRenderer {
    Box::new(
        move |container_data: ContainerRenderData,
              _callbacks: ContainerCallbacks,
              children: AnyView| {
            let total_fields = container_data.total_fields;
            let total_errors = container_data.total_errors;
            let _is_valid = container_data.is_valid;

            view! {
                <div class="bg-white border border-gray-200 rounded-lg overflow-hidden">
                    // Simple header
                    <div class="px-4 py-3 bg-gray-50 border-b border-gray-200 flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            <h3 class="text-lg font-medium text-gray-900">
                                {title.clone().unwrap_or_else(|| "Custom Fields".to_string())}
                            </h3>
                            <span class="text-sm text-gray-500">
                                "(" {total_fields} " fields"
                                {if total_errors > 0 {
                                    format!(", {} errors", total_errors)
                                } else {
                                    String::new()
                                }} ")"
                            </span>
                        </div>

                        <button
                            type="button"
                            class="text-gray-400 hover:text-gray-600"
                            on:click=move |_| {
                                is_collapsed.update(|collapsed| *collapsed = !*collapsed)
                            }
                        >
                            <svg
                                class=format!(
                                    "h-5 w-5 transition-transform {}",
                                    if is_collapsed.get() { "rotate-0" } else { "rotate-180" },
                                )
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M19 9l-7 7-7-7"
                                />
                            </svg>
                        </button>
                    </div>

                    // Content area - simple show/hide
                    {if !is_collapsed.get() {
                        Some(
                            view! {
                                <div class="p-4">
                                    // Fields in a flex wrap layout similar to the form
                                    <div class="flex flex-wrap items-start gap-x-6 gap-y-4 w-full">
                                        {children}
                                    </div>

                                // // Simple action buttons
                                // <div class="mt-6 pt-4 border-t border-gray-200 flex items-center justify-between">
                                // <div class="text-sm">
                                // {if !is_valid {
                                // view! {
                                // <span class="text-red-600">
                                // "Please fix validation errors before saving"
                                // </span>
                                // }
                                // .into_any()
                                // } else {
                                // view! {
                                // <span class="text-green-600">"All fields are valid"</span>
                                // }
                                // .into_any()
                                // }}
                                // </div>
                                // </div>
                                </div>
                            },
                        )
                    } else {
                        None
                    }}
                </div>
            }.into_any()
        },
    )
}
