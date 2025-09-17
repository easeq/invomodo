use crate::components::invoice::FieldCategory;
use std::collections::HashMap;

use super::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn FieldsRenderer(
    #[prop()] fields: ReadSignal<Vec<FieldItem>>,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);

    let line_item_fields = Memo::new(move |_| {
        fields
            .get()
            .into_iter()
            .filter(|f| f.category == FieldCategory::LineItem)
            .collect::<Vec<_>>()
    });

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
                    <For
                        each=move || line_item_fields.get()
                        key=|field| field.id.clone()
                        children=move |field| {
                            let field_id = field.id.clone();
                            let field_label = field.name.clone();
                            let required = field.required;
                            let default_value = field.default_value.clone();
                            let default_checked = field.default_checked;
                            let field_options = field.options.clone();
                            let value_memo = Memo::new({
                                let field_id = field_id.clone();
                                move |_| match form_values.get().get(&field_id).map(|f| &f.value) {
                                    Some(FieldValue::Text(v))
                                    | Some(FieldValue::Email(v))
                                    | Some(FieldValue::Phone(v))
                                    | Some(FieldValue::Textarea(v))
                                    | Some(FieldValue::Date(v))
                                    | Some(FieldValue::Dropdown(v)) => v.clone(),
                                    Some(FieldValue::Number(n)) => n.to_string(),
                                    _ => default_value.clone(),
                                }
                            });
                            let checked_memo = Memo::new({
                                let field_id = field_id.clone();
                                move |_| match form_values.get().get(&field_id).map(|f| &f.value) {
                                    Some(FieldValue::Checkbox(v)) => *v,
                                    _ => default_checked,
                                }
                            });
                            let on_change = {
                                let field_id = field_id.clone();
                                Callback::new({
                                    let field_type = field.field_type.clone();
                                    move |ev: web_sys::Event| {
                                        let input = ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .ok();
                                        let select = ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlSelectElement>()
                                            .ok();
                                        let textarea = ev
                                            .target()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlTextAreaElement>()
                                            .ok();
                                        form_values
                                            .update(|map| {
                                                if let Some(f) = map.get_mut(&field_id) {
                                                    f.value = match field_type {
                                                        FieldType::Text
                                                        | FieldType::Email
                                                        | FieldType::Phone
                                                        | FieldType::Date => {
                                                            FieldValue::Text(
                                                                input.as_ref().map(|i| i.value()).unwrap_or_default(),
                                                            )
                                                        }
                                                        FieldType::Textarea => {
                                                            FieldValue::Textarea(
                                                                textarea.as_ref().map(|t| t.value()).unwrap_or_default(),
                                                            )
                                                        }
                                                        FieldType::Number => {
                                                            let parsed = input
                                                                .as_ref()
                                                                .map(|i| i.value().parse::<f64>().unwrap_or(0.0))
                                                                .unwrap_or(0.0);
                                                            FieldValue::Number(parsed)
                                                        }
                                                        FieldType::Dropdown => {
                                                            FieldValue::Dropdown(
                                                                select.as_ref().map(|s| s.value()).unwrap_or_default(),
                                                            )
                                                        }
                                                        FieldType::Checkbox => {
                                                            FieldValue::Checkbox(
                                                                input.as_ref().map(|i| i.checked()).unwrap_or(false),
                                                            )
                                                        }
                                                    };
                                                }
                                            });
                                    }
                                })
                            };

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
                                                <TextInputField
                                                    field_id=field_id.clone()
                                                    input_type=field.field_type.to_input_type().to_string()
                                                    value=value_memo
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Number => {
                                            view! {
                                                <NumberInputField
                                                    field_id=field_id.clone()
                                                    value=value_memo
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Dropdown => {
                                            let options = field_options
                                                .unwrap_or_default()
                                                .split(',')
                                                .map(|s| s.trim().to_string())
                                                .collect::<Vec<String>>();

                                            view! {
                                                <DropdownField
                                                    field_id=field_id.clone()
                                                    options=options
                                                    value=value_memo
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Date => {
                                            view! {
                                                <DateInputField
                                                    field_id=field_id.clone()
                                                    value=value_memo
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Checkbox => {
                                            view! {
                                                <CheckboxField
                                                    field_id=field_id.clone()
                                                    checked=checked_memo
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Textarea => {
                                            view! {
                                                <TextareaField
                                                    field_id=field_id.clone()
                                                    value=value_memo
                                                    required=required
                                                    on_change=on_change
                                                />
                                            }
                                                .into_any()
                                        }
                                    }}
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TextInputField(
    #[prop()] field_id: String,
    #[prop()] input_type: String,
    #[prop()] value: Memo<String>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <input
            class="form-input"
            id=field_id.clone()
            name=field_id.clone()
            type=input_type.clone()
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(ev)
        />
    }
}

#[component]
pub fn NumberInputField(
    #[prop()] field_id: String,
    #[prop()] value: Memo<String>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <input
            class="form-input"
            id=field_id.clone()
            name=field_id.clone()
            type="number"
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(ev)
        />
    }
}

#[component]
pub fn DropdownField(
    #[prop()] field_id: String,
    #[prop()] options: Vec<String>,
    #[prop()] value: Memo<String>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <select
            class="form-select"
            id=field_id.clone()
            name=field_id.clone()
            prop:value=move || value.get()
            on:change=move |ev| on_change.run(ev)
        >
            <option value="" disabled selected>
                "Select an option"
            </option>
            <For
                each=move || options.clone()
                key=|opt| opt.clone()
                children=move |option| {
                    view! { <option value=option.clone()>{option.clone()}</option> }
                }
            />
        </select>
    }
}

#[component]
pub fn DateInputField(
    #[prop()] field_id: String,
    #[prop()] value: Memo<String>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <input
            type="date"
            class="form-input"
            id=field_id.clone()
            name=field_id.clone()
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(ev)
        />
    }
}

#[component]
pub fn CheckboxField(
    #[prop()] field_id: String,
    #[prop()] checked: Memo<bool>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class="form-checkbox"
            id=field_id.clone()
            name=field_id.clone()
            prop:checked=move || checked.get()
            on:change=move |ev| on_change.run(ev)
        />
    }
}

#[component]
pub fn TextareaField(
    #[prop()] field_id: String,
    #[prop()] value: Memo<String>,
    #[prop()] required: bool,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <textarea
            class="form-input"
            id=field_id.clone()
            name=field_id.clone()
            required=required
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(ev)
        ></textarea>
    }
}
