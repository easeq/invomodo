use crate::components::invoice::{FieldItem, FieldItemValue, FieldType, FieldValue};
use std::collections::HashMap;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Defines how fields should be grouped and laid out
#[derive(Clone, Debug, PartialEq)]
pub struct FieldGroup {
    pub name: Option<String>,
    pub fields: Vec<String>,   // Field IDs that belong to this group
    pub columns: usize,        // Number of columns for this group
    pub class: Option<String>, // Optional CSS class for the group
}

/// Column span configuration for individual fields
#[derive(Clone, Debug)]
pub struct FieldSpan {
    pub field_id: String,
    pub span: usize, // How many columns this field should span
}

/// Layout configuration with grouping support
#[derive(Clone, Debug)]
pub struct LayoutConfig {
    pub container_class: String,
    pub group_class: String,
    pub group_title_class: String,
    pub field_wrapper_class: String,
    pub groups: Vec<FieldGroup>,
    pub field_spans: Vec<FieldSpan>,
    pub default_columns: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            container_class: "w-full space-y-6".to_string(),
            group_class: "space-y-4".to_string(),
            group_title_class: "text-lg font-medium text-gray-900 border-b border-gray-200 pb-2"
                .to_string(),
            field_wrapper_class: "flex flex-col".to_string(),
            groups: vec![],
            field_spans: vec![],
            default_columns: 2,
        }
    }
}

/// Render mode determines how the fields container is displayed
#[derive(Clone, Debug, PartialEq)]
pub enum RenderMode {
    Collapsible {
        show_text: String,
        hide_text: String,
        initially_open: bool,
    },
    Inline,
    Card {
        title: Option<String>,
        collapsible: bool,
    },
    Custom,
}

/// Filter function type for custom field filtering
pub type FieldFilter = Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

/// Configuration for the fields renderer
#[derive(Clone)]
pub struct FieldsConfig {
    pub render_mode: RenderMode,
    pub layout: LayoutConfig,
    pub show_required_indicator: bool,
    pub required_indicator_class: String,
}

impl Default for FieldsConfig {
    fn default() -> Self {
        Self {
            render_mode: RenderMode::Inline,
            layout: LayoutConfig::default(),
            show_required_indicator: true,
            required_indicator_class: "text-red-500 ml-1".to_string(),
        }
    }
}

/// The main generic component for rendering custom fields with grouping
#[component]
pub fn FieldsRenderer(
    #[prop()] fields: ReadSignal<Vec<FieldItem>>,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
    #[prop(optional)] config: Option<FieldsConfig>,
    #[prop(optional)] filter: Option<FieldFilter>,
    #[prop(optional)] custom_wrapper: Option<ChildrenFn>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let is_open = RwSignal::new(matches!(
        config.render_mode,
        RenderMode::Collapsible {
            initially_open: true,
            ..
        } | RenderMode::Card { .. }
            | RenderMode::Inline
            | RenderMode::Custom
    ));

    // Apply filtering based on provided filter function or default to all fields
    let filtered_fields = Memo::new({
        let filter = filter;
        move |_| {
            let all_fields = fields.get();
            if let Some(ref filter_fn) = filter {
                all_fields
                    .into_iter()
                    .filter(|f| filter_fn(f))
                    .collect::<Vec<_>>()
            } else {
                all_fields
            }
        }
    });

    // Create a map for quick field lookup
    let fields_map = Memo::new(move |_| {
        filtered_fields
            .get()
            .into_iter()
            .map(|f| (f.id.clone(), f))
            .collect::<HashMap<String, FieldItem>>()
    });

    // Group fields according to configuration
    let grouped_fields = Memo::new({
        let config = config.clone();
        move |_| {
            let fields_map = fields_map.get();
            let mut result = Vec::new();
            let mut used_fields = std::collections::HashSet::new();

            // Process defined groups
            for group in &config.layout.groups {
                let group_fields: Vec<FieldItem> = group
                    .fields
                    .iter()
                    .filter_map(|field_id| {
                        if let Some(field) = fields_map.get(field_id) {
                            used_fields.insert(field_id.clone());
                            Some(field.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                if !group_fields.is_empty() {
                    result.push((Some(group.clone()), group_fields));
                }
            }

            // Add remaining ungrouped fields
            let ungrouped_fields: Vec<FieldItem> = fields_map
                .values()
                .filter(|field| !used_fields.contains(&field.id))
                .cloned()
                .collect();

            if !ungrouped_fields.is_empty() {
                let default_group = FieldGroup {
                    name: None,
                    fields: vec![],
                    columns: config.layout.default_columns,
                    class: None,
                };
                result.push((Some(default_group), ungrouped_fields));
            }

            result
        }
    });

    let get_field_span = {
        let config = config.clone();
        move |field_id: &str| -> usize {
            config
                .layout
                .field_spans
                .iter()
                .find(|span| span.field_id == field_id)
                .map(|span| span.span)
                .unwrap_or(1)
        }
    };

    let fields_content = view! {
        <div class=config.layout.container_class>
            <For
                each=move || grouped_fields.get()
                key=|(group_opt, _)| {
                    group_opt
                        .as_ref()
                        .map(|g| g.name.clone().unwrap_or_else(|| "default".to_string()))
                        .unwrap_or_else(|| "ungrouped".to_string())
                }
                children={
                    let config = config.clone();
                    let get_field_span = get_field_span.clone();
                    move |(group_opt, group_fields)| {
                        let group = group_opt.unwrap();
                        let grid_cols_class = match group.columns {
                            1 => "grid-cols-1",
                            2 => "grid-cols-1 md:grid-cols-2",
                            3 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
                            4 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
                            5 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5",
                            _ => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
                        };
                        let final_grid_class = group
                            .class
                            .as_ref()
                            .map(|c| format!("grid gap-4 {}", c))
                            .unwrap_or_else(|| format!("grid gap-4 {}", grid_cols_class));

                        view! {
                            <div class=config
                                .layout
                                .group_class
                                .clone()>
                                {group
                                    .name
                                    .map(|title| {
                                        view! {
                                            <h3 class=config
                                                .layout
                                                .group_title_class
                                                .clone()>{title}</h3>
                                        }
                                    })} <div class=final_grid_class>
                                    <For
                                        each=move || group_fields.clone()
                                        key=|field| field.id.clone()
                                        children={
                                            let config = config.clone();
                                            let get_field_span = get_field_span.clone();
                                            move |field| {
                                                let span = get_field_span(&field.id);
                                                let span_class = match span {
                                                    1 => "col-span-1",
                                                    2 => "col-span-2",
                                                    3 => "col-span-3",
                                                    4 => "col-span-4",
                                                    5 => "col-span-5",
                                                    _ => "col-span-full",
                                                };

                                                view! {
                                                    <div class=format!(
                                                        "{} {}",
                                                        config.layout.field_wrapper_class,
                                                        span_class,
                                                    )>
                                                        <FieldComponent
                                                            field=field.clone()
                                                            form_values=form_values
                                                            config=config.clone()
                                                        />
                                                    </div>
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }
                    }
                }
            />
        </div>
    };

    let content = match custom_wrapper {
        Some(wrapper) => wrapper().into_any(),
        None => fields_content.into_any(),
    };

    match config.render_mode {
        RenderMode::Collapsible { show_text, hide_text, .. } => {
            view! {
                <div class="w-full">
                    <button
                        type="button"
                        class="text-sm text-indigo-600 font-medium hover:underline mb-4"
                        on:click=move |_| is_open.update(|v| *v = !*v)
                    >
                        {move || if is_open.get() { hide_text.clone() } else { show_text.clone() }}
                    </button>

                    <div
                        class="transition-all duration-300 ease-in-out"
                        class:hidden=move || !is_open.get()
                    >
                        {content}
                    </div>
                </div>
            }.into_any()
        },
        RenderMode::Inline => {
            view! { <div class="w-full">{content}</div> }.into_any()
        },
        RenderMode::Card { title, collapsible } => {
            view! {
                <div class="bg-white shadow-sm border border-gray-200 rounded-lg">
                    {title
                        .map(|t| {
                            view! {
                                <div class="px-6 py-4 border-b border-gray-200 flex items-center justify-between">
                                    <h3 class="text-lg font-medium text-gray-900">{t}</h3>
                                    {if collapsible {
                                        Some(
                                            view! {
                                                <button
                                                    type="button"
                                                    class="text-gray-400 hover:text-gray-600 transition-colors"
                                                    on:click=move |_| is_open.update(|v| *v = !*v)
                                                >
                                                    <svg
                                                        class="w-5 h-5 transition-transform"
                                                        class:rotate-180=move || is_open.get()
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
                                            },
                                        )
                                    } else {
                                        None
                                    }}
                                </div>
                            }
                        })} <div class="p-6" class:hidden=move || collapsible && !is_open.get()>
                        {content}
                    </div>
                </div>
            }.into_any()
        },
        RenderMode::Custom => content,
    }
}

/// Enhanced field component that accepts configuration
#[component]
fn FieldComponent(
    #[prop()] field: FieldItem,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
    #[prop()] config: FieldsConfig,
) -> impl IntoView {
    let field_id = field.id.clone();
    let required = field.required;

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
            _ => FieldValue::Text(field.default_value.clone()),
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
        <>
            <label class="form-label text-sm font-medium text-gray-700 mb-1" for=field_id.clone()>
                {field.name.clone()}
                {if required && config.show_required_indicator {
                    Some(view! { <span class=config.required_indicator_class>*</span> })
                } else {
                    None
                }}
            </label>
            {input_view}
        </>
    }
}

// Helper functions remain the same
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

// Input field components remain the same
#[component]
pub fn TextInputField(
    #[prop()] field_id: String,
    #[prop()] input_type: String,
    #[prop()] value: Memo<String>,
    #[prop()] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
    view! {
        <input
            class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
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
            class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
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
            class="form-select block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
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
            class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
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
            class="form-checkbox h-4 w-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
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
            class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
            id=field_id.clone()
            name=field_id.clone()
            rows="3"
            required=required
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(ev)
        ></textarea>
    }
}
