use super::{CustomFieldItem, FieldType};
use leptos::prelude::*;

#[component]
pub fn CustomFieldsRenderer(#[prop()] fields: Vec<CustomFieldItem>) -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <div class="w-full">
            <button
                type="button"
                class="text-sm text-indigo-600 font-medium hover:underline mb-2"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                {move || if is_open.get() { "Hide Custom Fields" } else { "Show Custom Fields" }}
            </button>

            <div
                class="transition-all duration-300 ease-in-out"
                class:hidden=move || !is_open.get()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {fields
                        .into_iter()
                        .map(|field| {
                            let field_id = field.id.clone();
                            let field_label = field.name.clone();
                            let required = field.required;

                            view! {
                                <div class="flex flex-col">
                                    <label class="form-label" for=field_id.clone()>
                                        {field_label.clone()}
                                        {if required {
                                            Some(view! { <span class="text-red-500 ml-1">*</span> })
                                        } else {
                                            None
                                        }}
                                    </label>

                                    {match field.field_type {
                                        FieldType::Text | FieldType::Email | FieldType::Phone => {
                                            view! {
                                                <input
                                                    class="form-input"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    type=match field.field_type {
                                                        FieldType::Email => "email",
                                                        FieldType::Phone => "tel",
                                                        _ => "text",
                                                    }
                                                    placeholder=field.default_value.clone()
                                                    required=required
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Number => {

                                            view! {
                                                <input
                                                    class="form-input"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    type="number"
                                                    step="any"
                                                    placeholder=field.default_value.clone()
                                                    required=required
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Dropdown => {

                                            view! {
                                                <select
                                                    class="form-select"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    required=required
                                                >
                                                    // You can customize this to load real options
                                                    <option value="">"Select an option"</option>
                                                    <option value="Option1">"Option 1"</option>
                                                    <option value="Option2">"Option 2"</option>
                                                </select>
                                            }
                                                .into_any()
                                        }
                                        FieldType::Date => {

                                            view! {
                                                <input
                                                    class="form-input"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    type="date"
                                                    required=required
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Checkbox => {

                                            view! {
                                                <input
                                                    class="form-checkbox h-5 w-5"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    type="checkbox"
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Textarea => {

                                            view! {
                                                <textarea
                                                    class="form-textarea"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    placeholder=field.default_value.clone()
                                                    required=required
                                                ></textarea>
                                            }
                                                .into_any()
                                        }
                                    }}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
