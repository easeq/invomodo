use crate::components::invoice::{FieldCategory, FieldItem, FieldItemValue, FieldType, FieldValue};
use std::collections::HashMap;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Render mode determines how the fields container is displayed
#[derive(Clone, Debug, PartialEq)]
pub enum RenderMode {
    /// Collapsible section with toggle button
    Collapsible {
        show_text: String,
        hide_text: String,
        initially_open: bool,
    },
    /// Always visible inline rendering
    Inline,
    /// Card-style rendering with optional header
    Card {
        title: Option<String>,
        collapsible: bool,
    },
    /// Custom wrapper with user-provided view function
    Custom,
}

/// Layout configuration for the fields grid
#[derive(Clone, Debug)]
pub struct LayoutConfig {
    pub container_class: String,
    pub grid_class: String,
    pub field_wrapper_class: String,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            container_class: "w-full".to_string(),
            grid_class: "grid grid-cols-1 md:grid-cols-2 gap-4".to_string(),
            field_wrapper_class: "flex flex-col".to_string(),
        }
    }
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

/// The main generic component for rendering custom fields
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

    let fields_content = view! {
        <div class=config.layout.grid_class>
            <For
                each=move || filtered_fields.get()
                key=|field| field.id.clone()
                children={
                    let config = config.clone();
                    move |field| {
                        view! {
                            <div class=config.layout.field_wrapper_class.clone()>
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
    };

    let content = match custom_wrapper {
        Some(wrapper) => wrapper().into_any(),
        None => fields_content.into_any(),
    };

    match config.render_mode {
        RenderMode::Collapsible { show_text, hide_text, .. } => {
            view! {
                <div class=config.layout.container_class>
                    <button
                        type="button"
                        class="text-sm text-indigo-600 font-medium hover:underline mb-2"
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
            view! { <div class=config.layout.container_class>{content}</div> }.into_any()
        },
        RenderMode::Card { title, collapsible } => {
            view! {
                <div class=format!(
                    "{} bg-white shadow-sm border border-gray-200 rounded-lg",
                    config.layout.container_class,
                )>
                    {title
                        .map(|t| {
                            view! {
                                <div class="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
                                    <h3 class="text-lg font-medium text-gray-900">{t}</h3>
                                    {if collapsible {
                                        Some(
                                            view! {
                                                <button
                                                    type="button"
                                                    class="text-gray-400 hover:text-gray-600"
                                                    on:click=move |_| is_open.update(|v| *v = !*v)
                                                >
                                                    <svg
                                                        class="w-5 h-5"
                                                        fill="none"
                                                        stroke="currentColor"
                                                        viewBox="0 0 24 24"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d=move || {
                                                                if is_open.get() {
                                                                    "M5 15l7-7 7 7"
                                                                } else {
                                                                    "M19 9l-7 7-7-7"
                                                                }
                                                            }
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
                        })} <div class="p-4" class:hidden=move || collapsible && !is_open.get()>
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
            <label class="form-label" for=field_id.clone()>
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

// Helper functions and utility components remain the same...
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

// All the input field components remain the same...
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
