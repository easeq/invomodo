use leptos::prelude::*;

use crate::components::editable_grid::{
    FormData, FormValidation, ItemData, ValidationResult, use_editable_grid, validation::validators,
};

// 1. Define your data structure for Tax
#[derive(Clone, PartialEq, Debug)]
pub struct TaxItem {
    pub id: String,
    pub name: String,
    pub rate: f64,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct TaxForm {
    pub name: String,
    pub rate: String, // Keep as string for form input handling
}

// 2. Implement required traits
impl FormData for TaxItem {
    type FormProps = TaxForm;

    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            rate: 0.0,
        }
    }

    fn to_form_props(&self) -> Self::FormProps {
        TaxForm {
            name: self.name.clone(),
            rate: self.rate.to_string(),
        }
    }

    fn from_form_props(props: &Self::FormProps) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: props.name.clone(),
            rate: props.rate.parse::<f64>().unwrap_or(0.0),
        }
    }
}

impl ItemData for TaxItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_title(&self) -> String {
        self.name.clone()
    }

    fn get_subtitle(&self) -> Option<String> {
        Some(format!("{}%", self.rate))
    }

    fn get_metadata(&self) -> Vec<(String, String)> {
        vec![
            ("rate".to_string(), self.rate.to_string()),
            ("id".to_string(), self.id.clone()),
        ]
    }
}

impl FormValidation for TaxForm {
    fn validate(&self) -> ValidationResult {
        let mut results = vec![
            validators::required(&self.name, "Tax Name"),
            validators::min_length(&self.name, 2, "Tax Name"),
            validators::max_length(&self.name, 50, "Tax Name"),
            validators::required(&self.rate, "Rate"),
        ];

        // Validate rate is a valid number
        if !self.rate.is_empty() {
            if self.rate.parse::<f64>().is_err() {
                results.push(ValidationResult::new().with_field_error(
                    "rate".to_string(),
                    "Rate must be a valid number".to_string(),
                ));
            } else if let Ok(rate) = self.rate.parse::<f64>() {
                if rate < 0.0 {
                    results.push(ValidationResult::new().with_field_error(
                        "rate".to_string(),
                        "Rate cannot be negative".to_string(),
                    ));
                } else if rate > 100.0 {
                    results.push(ValidationResult::new().with_field_error(
                        "rate".to_string(),
                        "Rate cannot exceed 100%".to_string(),
                    ));
                }
            }
        }

        validators::combine_results(results)
    }
}

// 3. Tax Management Component
#[component]
pub fn Taxes() -> impl IntoView {
    let initial_taxes = vec![
        TaxItem {
            id: "1".to_string(),
            name: "VAT".to_string(),
            rate: 10.00,
        },
        TaxItem {
            id: "2".to_string(),
            name: "Service Tax".to_string(),
            rate: 5.00,
        },
    ];

    let grid = use_editable_grid(initial_taxes);

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (rate_value, set_rate_value) = signal(String::new());

    // Update form fields when grid state changes
    Effect::new({
        move |_| {
            let form_state = grid.form_state.get();
            let form = form_state.current_form;
            set_name_value.set(form.name);
            set_rate_value.set(form.rate);
        }
    });

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form_data = TaxForm {
            name: name_value.get(),
            rate: rate_value.get(),
        };

        // Validate before submitting
        let validation = form_data.validate();
        if validation.is_valid {
            grid.actions.submit_form.run(form_data);
            // Clear form after successful submission
            set_name_value.set(String::new());
            set_rate_value.set(String::new());
        }
    };

    // Handle input changes
    let handle_name_input = move |ev| {
        let value = event_target_value(&ev);
        set_name_value.set(value.clone());

        let form_data = TaxForm {
            name: value,
            rate: rate_value.get(),
        };
        grid.actions.update_form.run(form_data);
    };

    let handle_rate_input = move |ev| {
        let value = event_target_value(&ev);
        set_rate_value.set(value.clone());

        let form_data = TaxForm {
            name: name_value.get(),
            rate: value,
        };
        grid.actions.update_form.run(form_data);
    };

    view! {
        <div class="container mx-auto p-4">
            // Tax Form Section
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">
                    {move || {
                        let form_state = grid.form_state.get();
                        if form_state.is_editing { "Edit Tax" } else { "Add New Tax" }
                    }}
                </h3>

                <form on:submit=handle_submit>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label class="form-label" for="tax-name">
                                "Tax Name"
                            </label>
                            <input
                                class="form-input"
                                id="tax-name"
                                name="tax-name"
                                placeholder="e.g., VAT"
                                type="text"
                                prop:value=move || name_value.get()
                                on:input=handle_name_input
                                required
                            />
                        </div>
                        <div>
                            <label class="form-label" for="tax-rate">
                                "Rate (%)"
                            </label>
                            <input
                                class="form-input"
                                id="tax-rate"
                                name="tax-rate"
                                placeholder="0.00"
                                type="number"
                                step="0.1"
                                min="0"
                                max="100"
                                prop:value=move || rate_value.get()
                                on:input=handle_rate_input
                                required
                            />
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
                                    set_rate_value.set(String::new());
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
                                if form_state.is_editing { "Update Tax" } else { "Add Tax" }
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Tax Table Section
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
                                                    <h3 class="font-semibold text-lg">{item.name.clone()}</h3>
                                                    <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">
                                                        {format!("{}%", item.rate)}
                                                    </span>
                                                </div>
                                                <div class="flex justify-end space-x-2 mt-3">
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
                                        }
                                    }
                                />
                                <Show when=move || grid.is_empty.get()>
                                    <div class="text-center py-12">
                                        <p class="text-sm text-gray-500">
                                            "No taxes found. Add your first tax using the form above."
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
                                                "Tax Name"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Rate (%)"
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
                                                            {format!("{:.2}", item.rate)}
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
                                                    colspan="3"
                                                >
                                                    "No taxes found. Add your first tax using the form above."
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
                    "Total taxes: " {move || grid.item_count.get()} " | Average rate: "
                    {move || {
                        let items = grid.items.get();
                        if items.is_empty() {
                            "0.00%".to_string()
                        } else {
                            let avg = items.iter().map(|i| i.data.rate).sum::<f64>()
                                / items.len() as f64;
                            format!("{:.2}%", avg)
                        }
                    }} " | Device: " {move || format!("{:?}", grid.device_type.get())}
                </div>
            </div>
        </div>
    }
}
