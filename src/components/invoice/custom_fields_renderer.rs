use crate::components::invoice::{FieldCategory, FieldItem, FieldItemValue, FieldType, FieldValue};
use std::collections::HashMap;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// The main component for rendering custom fields.
/// It filters fields and manages the visibility of the custom fields section.
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
                            view! { <FieldComponent field=field.clone() form_values=form_values /> }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// A component that renders a single custom field based on its type.
/// It encapsulates the logic for displaying the label and the correct input component.
#[component]
fn FieldComponent(
    #[prop()] field: FieldItem,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    let field_id = field.id.clone();
    let required = field.required;

    // Use a Memo to get the initial value from form_values.
    // This provides an efficient way to react to changes in form_values.
    let value_memo = Memo::new({
        let field_id = field.id.clone();
        move |_| match form_values.get().get(&field_id).map(|f| &f.value) {
            Some(FieldValue::Text(v))
            | Some(FieldValue::Email(v))
            | Some(FieldValue::Phone(v))
            | Some(FieldValue::Textarea(v))
            | Some(FieldValue::Date(v))
            | Some(FieldValue::Dropdown(v)) => FieldValue::Text(v.clone()),
            Some(FieldValue::Number(n)) => FieldValue::Number(*n),
            Some(FieldValue::Checkbox(b)) => FieldValue::Checkbox(*b),
            _ => FieldValue::Text(field.default_value.clone()), // Default to text for simplicity
        }
    });

    let on_change = {
        let field_id = field.id.clone();
        Callback::new({
            let field_type = field.field_type.clone();
            move |ev: web_sys::Event| {
                let value = get_event_value(ev, &field_type);
                form_values.update(|map| {
                    if let Some(f) = map.get_mut(&field_id) {
                        f.value = value;
                    }
                });
            }
        })
    };

    let input_view = match field.field_type {
        FieldType::Text | FieldType::Email | FieldType::Phone => view! {
            <TextInputField
                field_id=field_id.clone()
                input_type=field.field_type.to_input_type().to_string()
                value=Memo::new(move |_| {
                    match value_memo.get() {
                        FieldValue::Text(v) => v,
                        _ => String::new(),
                    }
                })
                on_change=on_change
            />
        }
        .into_any(),
        FieldType::Number => view! {
            <NumberInputField
                field_id=field_id.clone()
                value=Memo::new(move |_| {
                    match value_memo.get() {
                        FieldValue::Number(n) => n.to_string(),
                        _ => String::new(),
                    }
                })
                on_change=on_change
            />
        }
        .into_any(),
        FieldType::Dropdown => {
            let options = field
                .options
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            view! {
                <DropdownField
                    field_id=field_id.clone()
                    options=options
                    value=Memo::new(move |_| {
                        match value_memo.get() {
                            FieldValue::Text(v) => v,
                            _ => String::new(),
                        }
                    })
                    on_change=on_change
                />
            }
            .into_any()
        }
        FieldType::Date => view! {
            <DateInputField
                field_id=field_id.clone()
                value=Memo::new(move |_| {
                    match value_memo.get() {
                        FieldValue::Text(v) => v,
                        _ => String::new(),
                    }
                })
                on_change=on_change
            />
        }
        .into_any(),
        FieldType::Checkbox => view! {
            <CheckboxField
                field_id=field_id.clone()
                checked=Memo::new(move |_| {
                    match value_memo.get() {
                        FieldValue::Checkbox(b) => b,
                        _ => field.default_checked,
                    }
                })
                on_change=on_change
            />
        }
        .into_any(),
        FieldType::Textarea => view! {
            <TextareaField
                field_id=field_id.clone()
                value=Memo::new(move |_| {
                    match value_memo.get() {
                        FieldValue::Text(v) => v,
                        _ => String::new(),
                    }
                })
                required=required
                on_change=on_change
            />
        }
        .into_any(),
    };

    view! {
        <div class="flex flex-col">
            <label class="form-label" for=field_id.clone()>
                {field.name.clone()}
                {if required {
                    Some(view! { <span class="text-red-500 ml-1">*</span> })
                } else {
                    None
                }}
            </label>
            {input_view}
        </div>
    }
}

/// A helper function to extract the value from different event targets.
fn get_event_value(ev: web_sys::Event, field_type: &FieldType) -> FieldValue {
    match field_type {
        FieldType::Text | FieldType::Email | FieldType::Phone | FieldType::Date => {
            let input = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            FieldValue::Text(input.map(|i| i.value()).unwrap_or_default())
        }
        FieldType::Textarea => {
            let textarea = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok());
            FieldValue::Textarea(textarea.map(|t| t.value()).unwrap_or_default())
        }
        FieldType::Number => {
            let input = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            let parsed = input
                .map(|i| i.value().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            FieldValue::Number(parsed)
        }
        FieldType::Dropdown => {
            let select = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok());
            FieldValue::Dropdown(select.map(|s| s.value()).unwrap_or_default())
        }
        FieldType::Checkbox => {
            let input = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            FieldValue::Checkbox(input.map(|i| i.checked()).unwrap_or(false))
        }
    }
}

// Reusable input components (unmodified from the original)
// ... (The original TextInputField, NumberInputField, etc. components are
//      assumed to be in the same file or a separate shared file)
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
