use crate::components::invoice::FieldCategory;
use std::collections::HashMap;

use super::*;
use leptos::prelude::*;

#[component]
pub fn FieldsRenderer(
    #[prop()] fields: ReadSignal<Vec<FieldItem>>,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
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
                        .get()
                        .into_iter()
                        .filter(|f| f.category == FieldCategory::LineItem)
                        .map(|field| {
                            let field_id = field.id.clone();
                            let field_label = field.name.clone();
                            let required = field.required;
                            let default_value = field.default_value.clone();
                            let default_checked = field.default_checked;
                            let field_options = field.options.clone();

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
                                            let field_value = Memo::new({
                                                let field_id = field.id.clone();
                                                move |_| match form_values
                                                    .get()
                                                    .get(&field_id)
                                                    .map(|f| {
                                                        log::debug!("form_value: {f:#?}");
                                                        &f.value
                                                    })
                                                {
                                                    Some(
                                                        FieldValue::Text(v)
                                                        | FieldValue::Email(v)
                                                        | FieldValue::Phone(v),
                                                    ) => {
                                                        log::debug!("value: {v:#?}");
                                                        v.clone()
                                                    }
                                                    _ => {
                                                        log::debug!("default_value: {default_value:#?}");
                                                        default_value.clone()
                                                    }
                                                }
                                            });

                                            view! {
                                                <input
                                                    class="form-input"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    type="text"
                                                    prop:value=field_value.get()
                                                    on:input={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let new_val = event_target_value(&ev);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field_val) = map.get_mut(&field_id) {
                                                                        field_val.value = match &field_val.value {
                                                                            FieldValue::Text(_) => FieldValue::Text(new_val.clone()),
                                                                            FieldValue::Email(_) => FieldValue::Email(new_val.clone()),
                                                                            FieldValue::Phone(_) => FieldValue::Phone(new_val.clone()),
                                                                            _ => field_val.value.clone(),
                                                                        };
                                                                    }
                                                                });
                                                        }
                                                    }
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Number => {

                                            view! {
                                                <input
                                                    class="form-input"
                                                    type="number"
                                                    value={
                                                        let field_id = field_id.clone();
                                                        move || {
                                                            match form_values.get().get(&field_id) {
                                                                Some(
                                                                    FieldItemValue { value: FieldValue::Number(n), .. },
                                                                ) => n.to_string(),
                                                                _ => default_value.clone(),
                                                            }
                                                        }
                                                    }
                                                    on:input={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let val = event_target_value(&ev)
                                                                .parse::<f64>()
                                                                .unwrap_or(0.0);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field) = map.get_mut(&field_id) {
                                                                        field.value = FieldValue::Number(val);
                                                                    }
                                                                });
                                                        }
                                                    }
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Dropdown => {
                                            let options_list = field_options
                                                .unwrap_or_default()
                                                .split(',')
                                                .map(|s| s.trim().to_string())
                                                .collect::<Vec<String>>();

                                            view! {
                                                <select
                                                    class="form-select"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    prop:value={
                                                        let field_id = field_id.clone();
                                                        move || {
                                                            match form_values.get().get(&field_id) {
                                                                Some(
                                                                    FieldItemValue { value: FieldValue::Dropdown(v), .. },
                                                                ) => v.clone(),
                                                                _ => default_value.clone(),
                                                            }
                                                        }
                                                    }
                                                    on:change={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let val = event_target_value(&ev);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field) = map.get_mut(&field_id) {
                                                                        field.value = FieldValue::Dropdown(val);
                                                                    }
                                                                });
                                                        }
                                                    }
                                                >
                                                    <option value="" disabled selected>
                                                        "Select an option"
                                                    </option>
                                                    <For
                                                        each=move || options_list.clone()
                                                        key=|option| option.clone()
                                                        children=move |option| {
                                                            view! {
                                                                <option value=option.clone()>{option.clone()}</option>
                                                            }
                                                        }
                                                    />
                                                </select>
                                            }
                                                .into_any()
                                        }
                                        FieldType::Date => {

                                            view! {
                                                <input
                                                    type="date"
                                                    class="form-input"
                                                    value={
                                                        let field_id = field_id.clone();
                                                        move || {
                                                            match form_values.get().get(&field_id) {
                                                                Some(FieldItemValue { value: FieldValue::Date(d), .. }) => {
                                                                    d.clone()
                                                                }
                                                                _ => default_value.clone(),
                                                            }
                                                        }
                                                    }
                                                    on:input={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let val = event_target_value(&ev);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field) = map.get_mut(&field_id) {
                                                                        field.value = FieldValue::Date(val);
                                                                    }
                                                                });
                                                        }
                                                    }
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Checkbox => {

                                            view! {
                                                <input
                                                    type="checkbox"
                                                    class="form-checkbox"
                                                    checked={
                                                        let field_id = field_id.clone();
                                                        move || {
                                                            match form_values.get().get(&field_id) {
                                                                Some(
                                                                    FieldItemValue { value: FieldValue::Checkbox(v), .. },
                                                                ) => *v,
                                                                _ => default_checked,
                                                            }
                                                        }
                                                    }
                                                    on:change={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let checked = event_target_checked(&ev);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field) = map.get_mut(&field_id) {
                                                                        field.value = FieldValue::Checkbox(checked);
                                                                    }
                                                                });
                                                        }
                                                    }
                                                />
                                            }
                                                .into_any()
                                        }
                                        FieldType::Textarea => {
                                            view! {
                                                <textarea
                                                    class="form-input"
                                                    id=field_id.clone()
                                                    name=field_id.clone()
                                                    required=required
                                                    prop:value={
                                                        let field_id = field_id.clone();
                                                        move || match form_values
                                                            .get()
                                                            .get(&field_id)
                                                            .map(|f| &f.value)
                                                        {
                                                            Some(FieldValue::Textarea(v)) => v.clone(),
                                                            _ => default_value.clone(),
                                                        }
                                                    }
                                                    on:input={
                                                        let field_id = field_id.clone();
                                                        move |ev| {
                                                            let new_val = event_target_value(&ev);
                                                            form_values
                                                                .update(|map| {
                                                                    if let Some(field_val) = map.get_mut(&field_id) {
                                                                        field_val.value = FieldValue::Textarea(new_val);
                                                                    }
                                                                });
                                                        }
                                                    }
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
