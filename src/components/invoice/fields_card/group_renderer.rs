use crate::components::invoice::FieldGroupLayout;
use crate::components::invoice::{GroupCallbacks, GroupRenderData, GroupRenderer};
use leptos::prelude::*;

pub fn create_group_renderer() -> GroupRenderer {
    Box::new(
        move |group_data: GroupRenderData, callbacks: GroupCallbacks, children: AnyView| {
            let group = group_data.group.clone();
            let is_expanded = group_data.is_expanded;
            let field_count = group_data.fields.len();
            let error_count = group_data
                .fields
                .iter()
                .map(|f| f.validation_errors.len())
                .sum::<usize>();

            view! {
                <div class="border border-gray-200 rounded-lg mb-4">
                    // Simple group header
                    <button
                        type="button"
                        class="w-full px-4 py-3 text-left bg-gray-50 border-b border-gray-200 hover:bg-gray-100 flex items-center justify-between"
                        on:click=move |_| {
                            if let Some(on_toggle) = &callbacks.on_expand_toggle {
                                on_toggle.run(group.id.clone());
                            }
                        }
                    >
                        <div>
                            <h4 class="font-medium text-gray-900">{group.name.clone()}</h4>
                            <div class="text-sm text-gray-500">
                                {field_count} " fields"
                                {if error_count > 0 {
                                    format!(", {} errors", error_count)
                                } else {
                                    String::new()
                                }}
                            </div>
                        </div>

                        <svg
                            class=format!(
                                "h-4 w-4 text-gray-400 transition-transform {}",
                                if is_expanded { "rotate-180" } else { "rotate-0" },
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

                    // Group content - simple show/hide
                    {if is_expanded {
                        Some(
                            view! {
                                <div class=format!(
                                    "p-4 {}",
                                    get_layout_classes(&group.layout),
                                )>{children}</div>
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

fn get_layout_classes(layout: &FieldGroupLayout) -> &str {
    match layout {
        FieldGroupLayout::Horizontal => "flex flex-wrap items-start gap-x-6 gap-y-4",
        FieldGroupLayout::Vertical => "space-y-4",
        FieldGroupLayout::Grid { columns } => match columns {
            1 => "grid grid-cols-1 gap-4",
            2 => "grid grid-cols-1 md:grid-cols-2 gap-4",
            3 => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
            4 => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4",
            _ => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
        },
    }
}
