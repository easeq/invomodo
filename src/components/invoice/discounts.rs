use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    editable_grid::{
        FormData, FormValidation, ItemData, ValidationResult, use_editable_grid,
        validation::validators,
    },
    ui::AutocompleteItem,
};

// 1. Define enums for discount type and scope
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum DiscountType {
    #[default]
    Percentage,
    FixedAmount,
}

impl std::fmt::Display for DiscountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscountType::FixedAmount => write!(f, "Fixed Amount"),
            DiscountType::Percentage => write!(f, "Percentage"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum DiscountScope {
    #[default]
    GlobalInvoice,
    LineItem,
}

impl std::fmt::Display for DiscountScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscountScope::LineItem => write!(f, "Line Item"),
            DiscountScope::GlobalInvoice => write!(f, "Global Invoice"),
        }
    }
}

// 2. Define your data structure for Discounts
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DiscountItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub discount_type: DiscountType,
    pub value: f64, // Amount for fixed, percentage for percentage
    pub scope: DiscountScope,
    pub is_default: bool,
}

impl AutocompleteItem for DiscountItem {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn search_text(&self) -> String {
        format!("{} {}", self.name, self.description)
    }

    fn display_text(&self) -> String {
        format!(
            "{} ({}{})",
            self.name,
            self.value,
            if self.discount_type == DiscountType::Percentage {
                "%"
            } else {
                "$"
            }
        )
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct DiscountForm {
    pub name: String,
    pub description: String,
    pub discount_type: DiscountType,
    pub value: String,
    pub scope: DiscountScope,
    pub is_default: bool,
}

// 3. Implement required traits
impl FormData for DiscountItem {
    type FormProps = DiscountForm;

    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            description: String::new(),
            discount_type: DiscountType::default(),
            value: 0.0,
            scope: DiscountScope::default(),
            is_default: false,
        }
    }

    fn to_form_props(&self) -> Self::FormProps {
        DiscountForm {
            name: self.name.clone(),
            description: self.description.clone(),
            discount_type: self.discount_type.clone(),
            value: self.value.to_string(),
            scope: self.scope.clone(),
            is_default: self.is_default,
        }
    }

    fn from_form_props(props: &Self::FormProps) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: props.name.clone(),
            description: props.description.clone(),
            discount_type: props.discount_type.clone(),
            value: props.value.parse::<f64>().unwrap_or(0.0),
            scope: props.scope.clone(),
            is_default: props.is_default,
        }
    }
}

impl ItemData for DiscountItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_title(&self) -> String {
        self.name.clone()
    }

    fn get_subtitle(&self) -> Option<String> {
        Some(self.description.clone())
    }

    fn get_metadata(&self) -> Vec<(String, String)> {
        let formatted_value = match self.discount_type {
            DiscountType::FixedAmount => format!("${:.2}", self.value),
            DiscountType::Percentage => format!("{}%", self.value),
        };

        vec![
            ("discount_type".to_string(), self.discount_type.to_string()),
            ("value".to_string(), formatted_value),
            ("scope".to_string(), self.scope.to_string()),
            ("is_default".to_string(), self.is_default.to_string()),
        ]
    }
}

impl FormValidation for DiscountForm {
    fn validate(&self) -> ValidationResult {
        let mut results = vec![
            validators::required(&self.name, "Discount Name"),
            validators::min_length(&self.name, 2, "Discount Name"),
            validators::max_length(&self.name, 100, "Discount Name"),
            validators::max_length(&self.description, 500, "Description"),
            validators::required(&self.value, "Value"),
        ];

        // Validate value is a valid number and within appropriate range
        if !self.value.is_empty() {
            if self.value.parse::<f64>().is_err() {
                results.push(ValidationResult::new().with_field_error(
                    "value".to_string(),
                    "Value must be a valid number".to_string(),
                ));
            } else if let Ok(value) = self.value.parse::<f64>() {
                if value < 0.0 {
                    results.push(ValidationResult::new().with_field_error(
                        "value".to_string(),
                        "Value cannot be negative".to_string(),
                    ));
                } else if matches!(self.discount_type, DiscountType::Percentage) && value > 100.0 {
                    results.push(ValidationResult::new().with_field_error(
                        "value".to_string(),
                        "Percentage cannot exceed 100%".to_string(),
                    ));
                }
            }
        }
        validators::combine_results(results)
    }
}

#[component]
pub fn Discounts(state: RwSignal<Vec<DiscountItem>>) -> impl IntoView {
    let grid = use_editable_grid(state.read_only(), state.write_only());

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (description_value, set_description_value) = signal(String::new());
    let (discount_type_value, set_discount_type_value) = signal(DiscountType::Percentage);
    let (value_value, set_value_value) = signal(String::new());
    let (scope_value, set_scope_value) = signal(DiscountScope::GlobalInvoice);
    let (is_default_value, set_is_default_value) = signal(false);

    // Update form fields when grid state changes
    Effect::new({
        move |_| {
            let form_state = grid.form_state.get();
            let form = form_state.current_form;
            set_name_value.set(form.name);
            set_description_value.set(form.description);
            set_discount_type_value.set(form.discount_type);
            set_value_value.set(form.value);
            set_scope_value.set(form.scope);
            set_is_default_value.set(form.is_default);
        }
    });

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form_data = DiscountForm {
            name: name_value.get(),
            description: description_value.get(),
            discount_type: discount_type_value.get(),
            value: value_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        };

        // Validate before submitting
        let validation = form_data.validate();
        if validation.is_valid {
            grid.actions.submit_form.run(form_data);
            // Clear form after successful submission
            set_name_value.set(String::new());
            set_description_value.set(String::new());
            set_discount_type_value.set(DiscountType::Percentage);
            set_value_value.set(String::new());
            set_scope_value.set(DiscountScope::GlobalInvoice);
            set_is_default_value.set(false);
        }
    };

    // Handle input changes
    let handle_name_input = move |ev| {
        let value = event_target_value(&ev);
        set_name_value.set(value.clone());
        grid.actions.update_form.run(DiscountForm {
            name: value,
            description: description_value.get(),
            discount_type: discount_type_value.get(),
            value: value_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_description_input = move |ev| {
        let value = event_target_value(&ev);
        set_description_value.set(value.clone());
        grid.actions.update_form.run(DiscountForm {
            name: name_value.get(),
            description: value,
            discount_type: discount_type_value.get(),
            value: value_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_discount_type_change = move |ev| {
        let value = event_target_value(&ev);
        let discount_type = if value == "FixedAmount" {
            DiscountType::FixedAmount
        } else {
            DiscountType::Percentage
        };
        set_discount_type_value.set(discount_type.clone());
        grid.actions.update_form.run(DiscountForm {
            name: name_value.get(),
            description: description_value.get(),
            discount_type,
            value: value_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_value_input = move |ev| {
        let value = event_target_value(&ev);
        set_value_value.set(value.clone());
        grid.actions.update_form.run(DiscountForm {
            name: name_value.get(),
            description: description_value.get(),
            discount_type: discount_type_value.get(),
            value,
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_scope_change = move |ev| {
        let value = event_target_value(&ev);
        let scope = if value == "LineItem" {
            DiscountScope::LineItem
        } else {
            DiscountScope::GlobalInvoice
        };
        set_scope_value.set(scope.clone());
        grid.actions.update_form.run(DiscountForm {
            name: name_value.get(),
            description: description_value.get(),
            discount_type: discount_type_value.get(),
            value: value_value.get(),
            scope,
            is_default: is_default_value.get(),
        });
    };

    let handle_is_default_change = move |ev| {
        let checked = event_target_checked(&ev);
        set_is_default_value.set(checked);
        grid.actions.update_form.run(DiscountForm {
            name: name_value.get(),
            description: description_value.get(),
            discount_type: discount_type_value.get(),
            value: value_value.get(),
            scope: scope_value.get(),
            is_default: checked,
        });
    };

    view! {
        <div class="container mx-auto p-4">
            // Discount Form Section
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">
                    {move || {
                        let form_state = grid.form_state.get();
                        if form_state.is_editing { "Edit Discount" } else { "Add New Discount" }
                    }}
                </h3>

                <form on:submit=handle_submit>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div>
                            <label class="form-label" for="discount-name">
                                "Discount Name"
                            </label>
                            <input
                                class="form-input"
                                id="discount-name"
                                name="discount-name"
                                placeholder="e.g., Early Bird"
                                type="text"
                                prop:value=move || name_value.get()
                                on:input=handle_name_input
                                required
                            />
                        </div>
                        <div class="md:col-span-2 lg:col-span-1">
                            <label class="form-label" for="discount-description">
                                "Description"
                            </label>
                            <input
                                class="form-input"
                                id="discount-description"
                                name="discount-description"
                                placeholder="A brief description of the discount"
                                type="text"
                                prop:value=move || description_value.get()
                                on:input=handle_description_input
                            />
                        </div>
                        <div>
                            <label class="form-label" for="discount-type">
                                "Type"
                            </label>
                            <select
                                class="form-select"
                                id="discount-type"
                                name="discount-type"
                                prop:value=move || {
                                    if matches!(
                                        discount_type_value.get(),
                                        DiscountType::FixedAmount
                                    ) {
                                        "FixedAmount"
                                    } else {
                                        "Percentage"
                                    }
                                }
                                on:change=handle_discount_type_change
                            >
                                <option value="Percentage">"Percentage (%)"</option>
                                <option value="FixedAmount">"Fixed Amount ($)"</option>
                            </select>
                        </div>
                        <div>
                            <label class="form-label" for="discount-value">
                                "Value"
                            </label>
                            <input
                                class="form-input"
                                id="discount-value"
                                name="discount-value"
                                placeholder="0.00"
                                type="number"
                                step="any"
                                min="0"
                                max=move || {
                                    if matches!(
                                        discount_type_value.get(),
                                        DiscountType::Percentage
                                    ) {
                                        "100"
                                    } else {
                                        ""
                                    }
                                }
                                prop:value=move || value_value.get()
                                on:input=handle_value_input
                                required
                            />
                        </div>
                        <div>
                            <label class="form-label" for="discount-scope">
                                "Scope"
                            </label>
                            <select
                                class="form-select"
                                id="discount-scope"
                                name="discount-scope"
                                prop:value=move || {
                                    if matches!(scope_value.get(), DiscountScope::LineItem) {
                                        "LineItem"
                                    } else {
                                        "GlobalInvoice"
                                    }
                                }
                                on:change=handle_scope_change
                            >
                                <option value="GlobalInvoice">"Global Invoice"</option>
                                <option value="LineItem">"Line Item"</option>
                            </select>
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
                                    set_description_value.set(String::new());
                                    set_discount_type_value.set(DiscountType::Percentage);
                                    set_value_value.set(String::new());
                                    set_scope_value.set(DiscountScope::GlobalInvoice);
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
                                if form_state.is_editing {
                                    "Update Discount"
                                } else {
                                    "Add Discount"
                                }
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Discount Table Section
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
                                                            {item.description.clone()}
                                                        </p>
                                                    </div>
                                                    <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">
                                                        {match item.discount_type {
                                                            DiscountType::Percentage => format!("{}%", item.value),
                                                            DiscountType::FixedAmount => format!("${:.2}", item.value),
                                                        }}
                                                    </span>
                                                </div>
                                                <div class="flex justify-between items-center mt-3 text-sm text-gray-600">
                                                    <span>"Scope: " {item.scope.to_string()}</span>
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
                                            "No discounts found. Add your first discount using the form above."
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
                                                "Discount Name"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Description"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Type"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Value"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Scope"
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
                                                            {item.description.clone()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {item.discount_type.to_string()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {match item.discount_type {
                                                                DiscountType::Percentage => format!("{:.2}%", item.value),
                                                                DiscountType::FixedAmount => format!("${:.2}", item.value),
                                                            }}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {item.scope.to_string()}
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
                                                    "No discounts found. Add your first discount using the form above."
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
                    "Total discounts: " {move || grid.item_count.get()} " | Device: "
                    {move || format!("{:?}", grid.device_type.get())}
                </div>
            </div>
        </div>
    }
}
