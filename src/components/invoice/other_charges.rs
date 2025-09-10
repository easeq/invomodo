use leptos::prelude::*;

use crate::components::editable_grid::{
    FormData, FormValidation, ItemData, ValidationResult, use_editable_grid, validation::validators,
};

// 1. Define enums for charge scope
#[derive(Clone, PartialEq, Debug, Default)]
pub enum ChargeScope {
    #[default]
    GlobalInvoice,
    LineItem,
}

impl std::fmt::Display for ChargeScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeScope::LineItem => write!(f, "Line Item"),
            ChargeScope::GlobalInvoice => write!(f, "Global Invoice"),
        }
    }
}

// 2. Define your data structure for Other Charges
#[derive(Clone, PartialEq, Debug)]
pub struct ChargeItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub amount: f64,
    pub scope: ChargeScope,
    pub is_default: bool,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ChargeForm {
    pub name: String,
    pub description: String,
    pub amount: String,
    pub scope: ChargeScope,
    pub is_default: bool,
}

// 3. Implement required traits
impl FormData for ChargeItem {
    type FormProps = ChargeForm;

    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            description: String::new(),
            amount: 0.0,
            scope: ChargeScope::default(),
            is_default: false,
        }
    }

    fn to_form_props(&self) -> Self::FormProps {
        ChargeForm {
            name: self.name.clone(),
            description: self.description.clone(),
            amount: self.amount.to_string(),
            scope: self.scope.clone(),
            is_default: self.is_default,
        }
    }

    fn from_form_props(props: &Self::FormProps) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: props.name.clone(),
            description: props.description.clone(),
            amount: props.amount.parse::<f64>().unwrap_or(0.0),
            scope: props.scope.clone(),
            is_default: props.is_default,
        }
    }
}

impl ItemData for ChargeItem {
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
        vec![
            ("amount".to_string(), self.amount.to_string()),
            ("scope".to_string(), self.scope.to_string()),
            ("is_default".to_string(), self.is_default.to_string()),
        ]
    }
}

impl FormValidation for ChargeForm {
    fn validate(&self) -> ValidationResult {
        let mut results = vec![
            validators::required(&self.name, "Charge Name"),
            validators::min_length(&self.name, 2, "Charge Name"),
            validators::max_length(&self.name, 50, "Charge Name"),
            validators::required(&self.amount, "Amount"),
        ];

        // Validate amount is a valid number and not negative
        if !self.amount.is_empty() {
            if self.amount.parse::<f64>().is_err() {
                results.push(ValidationResult::new().with_field_error(
                    "amount".to_string(),
                    "Amount must be a valid number".to_string(),
                ));
            } else if self.amount.parse::<f64>().unwrap_or(0.0) < 0.0 {
                results.push(ValidationResult::new().with_field_error(
                    "amount".to_string(),
                    "Amount cannot be negative".to_string(),
                ));
            }
        }
        validators::combine_results(results)
    }
}

// 4. Other Charges Management Component
#[component]
pub fn OtherCharges() -> impl IntoView {
    let initial_other_charges = vec![
        ChargeItem {
            id: "1".to_string(),
            name: "Shipping Fee".to_string(),
            description: "Standard shipping charge".to_string(),
            amount: 5.00,
            scope: ChargeScope::GlobalInvoice,
            is_default: true,
        },
        ChargeItem {
            id: "2".to_string(),
            name: "Handling Fee".to_string(),
            description: "Fee for handling fragile items".to_string(),
            amount: 10.00,
            scope: ChargeScope::LineItem,
            is_default: false,
        },
        ChargeItem {
            id: "3".to_string(),
            name: "Rush Order Fee".to_string(),
            description: "Fee for expedited order processing".to_string(),
            amount: 25.00,
            scope: ChargeScope::GlobalInvoice,
            is_default: false,
        },
    ];

    let grid = use_editable_grid(initial_other_charges);

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (description_value, set_description_value) = signal(String::new());
    let (amount_value, set_amount_value) = signal(String::new());
    let (scope_value, set_scope_value) = signal(ChargeScope::GlobalInvoice);
    let (is_default_value, set_is_default_value) = signal(false);

    // Update form fields when grid state changes
    Effect::new({
        move |_| {
            let form_state = grid.form_state.get();
            let form = form_state.current_form;
            set_name_value.set(form.name);
            set_description_value.set(form.description);
            set_amount_value.set(form.amount);
            set_scope_value.set(form.scope);
            set_is_default_value.set(form.is_default);
        }
    });

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form_data = ChargeForm {
            name: name_value.get(),
            description: description_value.get(),
            amount: amount_value.get(),
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
            set_amount_value.set(String::new());
            set_scope_value.set(ChargeScope::GlobalInvoice);
            set_is_default_value.set(false);
        }
    };

    // Handle input changes
    let handle_name_input = move |ev| {
        let value = event_target_value(&ev);
        set_name_value.set(value.clone());
        grid.actions.update_form.run(ChargeForm {
            name: value,
            description: description_value.get(),
            amount: amount_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_description_input = move |ev| {
        let value = event_target_value(&ev);
        set_description_value.set(value.clone());
        grid.actions.update_form.run(ChargeForm {
            name: name_value.get(),
            description: value,
            amount: amount_value.get(),
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_amount_input = move |ev| {
        let value = event_target_value(&ev);
        set_amount_value.set(value.clone());
        grid.actions.update_form.run(ChargeForm {
            name: name_value.get(),
            description: description_value.get(),
            amount: value,
            scope: scope_value.get(),
            is_default: is_default_value.get(),
        });
    };

    let handle_scope_change = move |ev| {
        let value = event_target_value(&ev);
        let scope = if value == "LineItem" {
            ChargeScope::LineItem
        } else {
            ChargeScope::GlobalInvoice
        };
        set_scope_value.set(scope.clone());
        grid.actions.update_form.run(ChargeForm {
            name: name_value.get(),
            description: description_value.get(),
            amount: amount_value.get(),
            scope,
            is_default: is_default_value.get(),
        });
    };

    let handle_is_default_change = move |ev| {
        let checked = event_target_checked(&ev);
        set_is_default_value.set(checked);
        grid.actions.update_form.run(ChargeForm {
            name: name_value.get(),
            description: description_value.get(),
            amount: amount_value.get(),
            scope: scope_value.get(),
            is_default: checked,
        });
    };

    view! {
        <div class="container mx-auto p-4">
            // Other Charges Form Section
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">
                    {move || {
                        let form_state = grid.form_state.get();
                        if form_state.is_editing { "Edit Charge" } else { "Add New Charge" }
                    }}
                </h3>

                <form on:submit=handle_submit>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div>
                            <label class="form-label" for="charge-name">
                                "Charge Name"
                            </label>
                            <input
                                class="form-input"
                                id="charge-name"
                                name="charge-name"
                                placeholder="e.g., Shipping Fee"
                                type="text"
                                prop:value=move || name_value.get()
                                on:input=handle_name_input
                                required
                            />
                        </div>
                        <div class="md:col-span-2 lg:col-span-1">
                            <label class="form-label" for="charge-description">
                                "Description"
                            </label>
                            <input
                                class="form-input"
                                id="charge-description"
                                name="charge-description"
                                placeholder="A brief description of the charge"
                                type="text"
                                prop:value=move || description_value.get()
                                on:input=handle_description_input
                            />
                        </div>
                        <div>
                            <label class="form-label" for="charge-amount">
                                "Amount"
                            </label>
                            <input
                                class="form-input"
                                id="charge-amount"
                                name="charge-amount"
                                placeholder="0.00"
                                type="number"
                                step="any"
                                min="0"
                                prop:value=move || amount_value.get()
                                on:input=handle_amount_input
                                required
                            />
                        </div>
                        <div>
                            <label class="form-label" for="charge-scope">
                                "Scope"
                            </label>
                            <select
                                class="form-select"
                                id="charge-scope"
                                name="charge-scope"
                                prop:value=move || {
                                    if matches!(scope_value.get(), ChargeScope::LineItem) {
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
                                    set_amount_value.set(String::new());
                                    set_scope_value.set(ChargeScope::GlobalInvoice);
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
                                if form_state.is_editing { "Update Charge" } else { "Add Charge" }
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Other Charges Table Section
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
                                                        {format!("${:.2}", item.amount)}
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
                                            "No charges found. Add your first charge using the form above."
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
                                                "Charge Name"
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
                                                "Amount"
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
                                                            {format!("${:.2}", item.amount)}
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
                                                    colspan="5"
                                                >
                                                    "No charges found. Add your first charge using the form above."
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
                    "Total charges: " {move || grid.item_count.get()} " | Device: "
                    {move || format!("{:?}", grid.device_type.get())}
                </div>
            </div>
        </div>
    }
}
