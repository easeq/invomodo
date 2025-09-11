use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::editable_grid::{
    FormData, FormValidation, ItemData, ValidationResult, use_editable_grid, validation::validators,
};

// 1. Define enums for custom field categories and types
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum FieldCategory {
    #[default]
    GlobalInvoice,
    BillerAddress,
    ClientAddress,
    LineItem,
    BillingDetails,
}

impl std::fmt::Display for FieldCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldCategory::BillerAddress => write!(f, "Biller Address"),
            FieldCategory::ClientAddress => write!(f, "Client Address"),
            FieldCategory::LineItem => write!(f, "Line Item"),
            FieldCategory::GlobalInvoice => write!(f, "Global Invoice"),
            FieldCategory::BillingDetails => write!(f, "Billing Details"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum FieldType {
    #[default]
    Text,
    Number,
    Email,
    Phone,
    Dropdown,
    Date,
    Checkbox,
    Textarea,
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Text => write!(f, "Text"),
            FieldType::Number => write!(f, "Number"),
            FieldType::Email => write!(f, "Email"),
            FieldType::Phone => write!(f, "Phone"),
            FieldType::Dropdown => write!(f, "Dropdown"),
            FieldType::Date => write!(f, "Date"),
            FieldType::Checkbox => write!(f, "Checkbox"),
            FieldType::Textarea => write!(f, "Textarea"),
        }
    }
}

// 2. Define your data structure for Custom Fields
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct CustomFieldItem {
    pub id: String,
    pub name: String,
    pub field_type: FieldType,
    pub category: FieldCategory,
    pub default_value: String,
    pub required: bool,
    pub is_default: bool,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CustomFieldForm {
    pub name: String,
    pub field_type: FieldType,
    pub category: FieldCategory,
    pub default_value: String,
    pub required: bool,
    pub is_default: bool,
}

// 3. Implement required traits
impl FormData for CustomFieldItem {
    type FormProps = CustomFieldForm;

    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            field_type: FieldType::default(),
            category: FieldCategory::default(),
            default_value: String::new(),
            required: false,
            is_default: false,
        }
    }

    fn to_form_props(&self) -> Self::FormProps {
        CustomFieldForm {
            name: self.name.clone(),
            field_type: self.field_type.clone(),
            category: self.category.clone(),
            default_value: self.default_value.clone(),
            required: self.required,
            is_default: self.is_default,
        }
    }

    fn from_form_props(props: &Self::FormProps) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: props.name.clone(),
            field_type: props.field_type.clone(),
            category: props.category.clone(),
            default_value: props.default_value.clone(),
            required: props.required,
            is_default: props.is_default,
        }
    }
}

impl ItemData for CustomFieldItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_title(&self) -> String {
        self.name.clone()
    }

    fn get_subtitle(&self) -> Option<String> {
        Some(self.default_value.clone())
    }

    fn get_metadata(&self) -> Vec<(String, String)> {
        vec![
            ("field_type".to_string(), self.field_type.to_string()),
            ("category".to_string(), self.category.to_string()),
            ("required".to_string(), self.required.to_string()),
            ("is_default".to_string(), self.is_default.to_string()),
        ]
    }
}

impl FormValidation for CustomFieldForm {
    fn validate(&self) -> ValidationResult {
        let results = vec![
            validators::required(&self.name, "Field Name"),
            validators::min_length(&self.name, 2, "Field Name"),
            validators::max_length(&self.name, 50, "Field Name"),
        ];

        validators::combine_results(results)
    }
}

// 4. Custom Field Management Component
#[component]
pub fn CustomFields(state: RwSignal<Vec<CustomFieldItem>>) -> impl IntoView {
    let grid = use_editable_grid(state.get());

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (field_type_value, set_field_type_value) = signal(FieldType::Text);
    let (category_value, set_category_value) = signal(FieldCategory::GlobalInvoice);
    let (default_value, set_default_value) = signal(String::new());
    let (required_value, set_required_value) = signal(false);
    let (is_default_value, set_is_default_value) = signal(false);

    // Update form fields when grid state changes
    Effect::new({
        move |_| {
            let form_state = grid.form_state.get();
            let form = form_state.current_form;
            set_name_value.set(form.name);
            set_field_type_value.set(form.field_type);
            set_category_value.set(form.category);
            set_default_value.set(form.default_value);
            set_required_value.set(form.required);
            set_is_default_value.set(form.is_default);
        }
    });

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form_data = CustomFieldForm {
            name: name_value.get(),
            field_type: field_type_value.get(),
            category: category_value.get(),
            default_value: default_value.get(),
            required: required_value.get(),
            is_default: is_default_value.get(),
        };

        let validation = form_data.validate();
        if validation.is_valid {
            grid.actions.submit_form.run(form_data);
            set_name_value.set(String::new());
            set_field_type_value.set(FieldType::Text);
            set_category_value.set(FieldCategory::GlobalInvoice);
            set_default_value.set(String::new());
            set_required_value.set(false);
            set_is_default_value.set(false);
        }
    };

    // Handle input changes
    let handle_name_input = move |ev| {
        let value = event_target_value(&ev);
        set_name_value.set(value.clone());
        grid.actions.update_form.run(CustomFieldForm {
            name: value,
            field_type: field_type_value.get(),
            category: category_value.get(),
            default_value: default_value.get(),
            required: required_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_field_type_change = move |ev| {
        let value = event_target_value(&ev);
        let field_type = match value.as_str() {
            "Number" => FieldType::Number,
            "Email" => FieldType::Email,
            "Phone" => FieldType::Phone,
            "Dropdown" => FieldType::Dropdown,
            "Date" => FieldType::Date,
            "Checkbox" => FieldType::Checkbox,
            "Textarea" => FieldType::Textarea,
            _ => FieldType::Text,
        };
        set_field_type_value.set(field_type.clone());
        grid.actions.update_form.run(CustomFieldForm {
            name: name_value.get(),
            field_type,
            category: category_value.get(),
            default_value: default_value.get(),
            required: required_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_category_change = move |ev| {
        let value = event_target_value(&ev);
        let category = match value.as_str() {
            "BillerAddress" => FieldCategory::BillerAddress,
            "ClientAddress" => FieldCategory::ClientAddress,
            "LineItem" => FieldCategory::LineItem,
            "BillingDetails" => FieldCategory::BillingDetails,
            _ => FieldCategory::GlobalInvoice,
        };
        set_category_value.set(category.clone());
        grid.actions.update_form.run(CustomFieldForm {
            name: name_value.get(),
            field_type: field_type_value.get(),
            category,
            default_value: default_value.get(),
            required: required_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_default_value_input = move |ev| {
        let value = event_target_value(&ev);
        set_default_value.set(value.clone());
        grid.actions.update_form.run(CustomFieldForm {
            name: name_value.get(),
            field_type: field_type_value.get(),
            category: category_value.get(),
            default_value: value,
            required: required_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_required_change = move |ev| {
        let checked = event_target_checked(&ev);
        set_required_value.set(checked);
        grid.actions.update_form.run(CustomFieldForm {
            name: name_value.get(),
            field_type: field_type_value.get(),
            category: category_value.get(),
            default_value: default_value.get(),
            required: checked,
            is_default: is_default_value.get(),
        });
    };

    let handle_is_default_change = move |ev| {
        let checked = event_target_checked(&ev);
        set_is_default_value.set(checked);
        grid.actions.update_form.run(CustomFieldForm {
            name: name_value.get(),
            field_type: field_type_value.get(),
            category: category_value.get(),
            default_value: default_value.get(),
            required: required_value.get(),
            is_default: checked,
        });
    };

    view! {
        <div class="container mx-auto p-4">
            // Custom Field Form Section
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">
                    {move || {
                        let form_state = grid.form_state.get();
                        if form_state.is_editing {
                            "Edit Custom Field"
                        } else {
                            "Add New Custom Field"
                        }
                    }}
                </h3>

                <form on:submit=handle_submit>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div>
                            <label class="form-label" for="field-name">
                                "Field Name"
                            </label>
                            <input
                                class="form-input"
                                id="field-name"
                                name="field-name"
                                placeholder="e.g., Due Date"
                                type="text"
                                prop:value=move || name_value.get()
                                on:input=handle_name_input
                                required
                            />
                        </div>
                        <div>
                            <label class="form-label" for="field-type">
                                "Field Type"
                            </label>
                            <select
                                class="form-select"
                                id="field-type"
                                name="field-type"
                                prop:value=move || field_type_value.get().to_string()
                                on:change=handle_field_type_change
                            >
                                <option value="Text">"Text"</option>
                                <option value="Number">"Number"</option>
                                <option value="Email">"Email"</option>
                                <option value="Phone">"Phone"</option>
                                <option value="Dropdown">"Dropdown"</option>
                                <option value="Date">"Date"</option>
                                <option value="Checkbox">"Checkbox"</option>
                                <option value="Textarea">"Textarea"</option>
                            </select>
                        </div>
                        <div>
                            <label class="form-label" for="field-category">
                                "Category"
                            </label>
                            <select
                                class="form-select"
                                id="field-category"
                                name="field-category"
                                prop:value=move || category_value.get().to_string()
                                on:change=handle_category_change
                            >
                                <option value="GlobalInvoice">"Global Invoice"</option>
                                <option value="BillerAddress">"Biller Address"</option>
                                <option value="ClientAddress">"Client Address"</option>
                                <option value="LineItem">"Line Item"</option>
                                <option value="BillingDetails">"Billing Details"</option>
                            </select>
                        </div>
                        <div class="md:col-span-2 lg:col-span-1">
                            <label class="form-label" for="default-value">
                                "Default Value"
                            </label>
                            <input
                                class="form-input"
                                id="default-value"
                                name="default-value"
                                placeholder="e.g., N/A"
                                type="text"
                                prop:value=move || default_value.get()
                                on:input=handle_default_value_input
                            />
                        </div>
                        <div class="flex items-end">
                            <label class="form-checkbox-label">
                                <input
                                    type="checkbox"
                                    class="form-checkbox"
                                    prop:checked=move || required_value.get()
                                    on:change=handle_required_change
                                />
                                <span class="ml-2 text-sm text-gray-700">"Required"</span>
                            </label>
                        </div>
                        <div class="flex items-end">
                            <label class="form-checkbox-label">
                                <input
                                    type="checkbox"
                                    class="form-checkbox"
                                    prop:checked=move || is_default_value.get()
                                    on:change=handle_is_default_change
                                />
                                <span class="ml-2 text-sm text-gray-700">"Set as Default"</span>
                            </label>
                        </div>
                    </div>
                    <div class="mt-6 flex justify-end gap-3">
                        <Show when=move || grid.form_state.get().is_editing>
                            <button
                                class="inline-flex justify-center rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                                type="button"
                                on:click=move |_| {
                                    grid.actions.cancel_edit.run(());
                                    set_name_value.set(String::new());
                                    set_field_type_value.set(FieldType::Text);
                                    set_category_value.set(FieldCategory::GlobalInvoice);
                                    set_default_value.set(String::new());
                                    set_required_value.set(false);
                                    set_is_default_value.set(false);
                                }
                            >
                                "Cancel"
                            </button>
                        </Show>
                        <button
                            class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                            type="submit"
                        >
                            {move || {
                                let form_state = grid.form_state.get();
                                if form_state.is_editing { "Update Field" } else { "Add Field" }
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Custom Field Table Section
            <div class="mt-8 bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
                {move || {
                    if grid.device_type.get().is_mobile() {
                        // Mobile card layout
                        view! {
                            <div class="p-4 space-y-4">
                                <For
                                    each=move || grid.items.get()
                                    key=|item_state| item_state.data.id.clone()
                                    children=move |item_state| {
                                        let index = item_state.index;
                                        let item = item_state.data.clone();
                                        view! {
                                            <div class="border rounded-lg p-4 bg-gray-50">
                                                <div class="flex justify-between items-start mb-2">
                                                    <div>
                                                        <h3 class="font-semibold text-lg">{item.name.clone()}</h3>
                                                        <p class="text-gray-500 text-sm">
                                                            {item.category.to_string()}
                                                        </p>
                                                    </div>
                                                    <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">
                                                        {item.field_type.to_string()}
                                                    </span>
                                                </div>
                                                <div class="flex justify-between items-center mt-3 text-sm text-gray-600">
                                                    <span>
                                                        "Required: " {if item.required { "Yes" } else { "No" }}
                                                    </span>
                                                    <div class="flex space-x-2">
                                                        <button
                                                            class="px-3 py-1 text-sm bg-indigo-600 text-white rounded hover:bg-indigo-700"
                                                            on:click=move |_| {
                                                                grid.actions.edit_item.run(index);
                                                            }
                                                        >
                                                            "Edit"
                                                        </button>
                                                        <button
                                                            class="px-3 py-1 text-sm bg-red-600 text-white rounded hover:bg-red-700"
                                                            on:click=move |_| {
                                                                grid.actions.delete_item.run(index);
                                                            }
                                                        >
                                                            "Delete"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                                <Show when=move || grid.is_empty.get()>
                                    <div class="text-center py-12">
                                        <p class="text-sm text-gray-500">
                                            "No custom fields found. Add your first field using the form above."
                                        </p>
                                    </div>
                                </Show>
                            </div>
                        }
                            .into_any()
                    } else {
                        // Desktop table layout
                        view! {
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Field Name"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Field Type"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Category"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Default Value"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Required"
                                            </th>
                                            <th class="relative px-6 py-3" scope="col">
                                                <span class="sr-only">"Actions"</span>
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        <For
                                            each=move || grid.items.get()
                                            key=|item_state| item_state.data.id.clone()
                                            children=move |item_state| {
                                                let index = item_state.index;
                                                let item = item_state.data.clone();
                                                view! {
                                                    <tr>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                            {item.name.clone()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {item.field_type.to_string()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {item.category.to_string()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {item.default_value.clone()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {if item.required { "Yes" } else { "No" }}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                            <button
                                                                class="text-indigo-600 hover:text-indigo-900 mr-4"
                                                                on:click=move |_| {
                                                                    grid.actions.edit_item.run(index);
                                                                }
                                                            >
                                                                "Edit"
                                                            </button>
                                                            <button
                                                                class="text-red-600 hover:text-red-900"
                                                                on:click=move |_| {
                                                                    grid.actions.delete_item.run(index);
                                                                }
                                                            >
                                                                "Delete"
                                                            </button>
                                                        </td>
                                                    </tr>
                                                }
                                            }
                                        />
                                        <Show when=move || grid.is_empty.get()>
                                            <tr>
                                                <td
                                                    class="px-6 py-12 text-center text-sm text-gray-500"
                                                    colspan="6"
                                                >
                                                    "No custom fields found. Add your first field using the form above."
                                                </td>
                                            </tr>
                                        </Show>
                                    </tbody>
                                </table>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>

            // Stats section
            <div class="mt-6 bg-gray-50 p-4 rounded-lg">
                <div class="text-sm text-gray-600">
                    "Total fields: " {move || grid.item_count.get()} " | Device: "
                    {move || format!("{:?}", grid.device_type.get())}
                </div>
            </div>
        </div>
    }
}
