use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::*;
use crate::components::editable_grid::{
    FormData, FormValidation, ItemData, ValidationResult, use_editable_grid, validation::validators,
};

// 1. Define your data structure for Line Items
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub taxes: Vec<TaxItem>,
    pub discounts: Vec<DiscountItem>,
    pub charges: Vec<ChargeItem>,
    pub custom_fields: Vec<(String, String)>, // (field_id, value)
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct LineItemForm {
    pub name: String,
    pub description: String,
    pub quantity: String,
    pub unit_price: String,
    pub taxes: Vec<TaxItem>, // IDs of selected taxes
    pub discounts: Vec<DiscountItem>,
    pub charges: Vec<ChargeItem>,
}

// 2. Implement required traits
impl FormData for LineItem {
    type FormProps = LineItemForm;

    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            description: String::new(),
            quantity: 0.0,
            unit_price: 0.0,
            taxes: Vec::new(),
            discounts: Vec::new(),
            charges: Vec::new(),
            custom_fields: Vec::new(),
        }
    }

    fn to_form_props(&self) -> Self::FormProps {
        LineItemForm {
            name: self.name.clone(),
            description: self.description.clone(),
            quantity: self.quantity.to_string(),
            unit_price: self.unit_price.to_string(),
            taxes: self.taxes.iter().map(|t| t.clone()).collect(),
            discounts: self.discounts.iter().map(|d| d.clone()).collect(),
            charges: self.charges.iter().map(|c| c.clone()).collect(),
        }
    }

    fn from_form_props(props: &Self::FormProps) -> Self {
        // In a real app, you would fetch the full item data from your state
        // based on the IDs. We'll use mock data here.
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: props.name.clone(),
            description: props.description.clone(),
            quantity: props.quantity.parse::<f64>().unwrap_or(0.0),
            unit_price: props.unit_price.parse::<f64>().unwrap_or(0.0),
            taxes: Vec::new(),
            discounts: Vec::new(),
            charges: Vec::new(),
            custom_fields: Vec::new(),
        }
    }
}

impl ItemData for LineItem {
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
            ("quantity".to_string(), self.quantity.to_string()),
            ("unit_price".to_string(), self.unit_price.to_string()),
        ]
    }
}

impl FormValidation for LineItemForm {
    fn validate(&self) -> ValidationResult {
        let mut results = vec![
            validators::required(&self.name, "Item Name"),
            validators::min_length(&self.name, 2, "Item Name"),
            validators::required(&self.quantity, "Quantity"),
            validators::required(&self.unit_price, "Unit Price"),
        ];

        if self.quantity.parse::<f64>().is_err() {
            results.push(ValidationResult::new().with_field_error(
                "quantity".to_string(),
                "Quantity must be a valid number".to_string(),
            ));
        }

        if self.unit_price.parse::<f64>().is_err() {
            results.push(ValidationResult::new().with_field_error(
                "unit-price".to_string(),
                "Unit Price must be a valid number".to_string(),
            ));
        }

        validators::combine_results(results)
    }
}

// 3. Line Items Management Component
#[component]
pub fn LineItems(
    state: RwSignal<Vec<LineItem>>,
    taxes: ReadSignal<Vec<TaxItem>>,
    discounts: ReadSignal<Vec<DiscountItem>>,
    charges: ReadSignal<Vec<ChargeItem>>,
) -> impl IntoView {
    let grid = use_editable_grid(state.read_only(), state.write_only());

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (description_value, set_description_value) = signal(String::new());
    let (quantity_value, set_quantity_value) = signal(String::new());
    let (unit_price_value, set_unit_price_value) = signal(String::new());
    let (selected_taxes, set_selected_taxes) = signal(Vec::<TaxItem>::new());
    let (selected_discounts, set_selected_discounts) = signal(Vec::<DiscountItem>::new());
    let (selected_charges, set_selected_charges) = signal(Vec::<ChargeItem>::new());

    // Computed signal for line item total
    let total = Signal::derive({
        move || {
            let form = grid.form_state.get().current_form;
            let quantity = form.quantity.parse::<f64>().unwrap_or(0.0);
            let unit_price = form.unit_price.parse::<f64>().unwrap_or(0.0);
            let mut base_total = quantity * unit_price;

            let form_taxes = form.taxes;
            let form_discounts = form.discounts;
            let form_charges = form.charges;

            // Apply taxes
            for form_tax in form_taxes {
                if let Some(tax) = taxes.get().iter().find(|t| t.id == form_tax.id) {
                    base_total += base_total * (tax.rate / 100.0);
                }
            }

            // Apply discounts
            for form_discount in form_discounts {
                if let Some(discount) = discounts.get().iter().find(|d| d.id == form_discount.id) {
                    match discount.discount_type {
                        DiscountType::Percentage => {
                            base_total -= base_total * (discount.value / 100.0)
                        }
                        DiscountType::FixedAmount => base_total -= discount.value,
                    }
                }
            }

            // Apply charges
            for form_charge in form_charges {
                if let Some(charge) = charges.get().iter().find(|c| c.id == form_charge.id) {
                    base_total += charge.amount;
                }
            }

            base_total
        }
    });

    // Update form fields when grid state changes
    Effect::new({
        move |_| {
            let form_state = grid.form_state.get();
            let form = form_state.current_form;
            set_name_value.set(form.name);
            set_description_value.set(form.description);
            set_quantity_value.set(form.quantity);
            set_unit_price_value.set(form.unit_price);
            set_selected_taxes.set(form.taxes);
            set_selected_discounts.set(form.discounts);
            set_selected_charges.set(form.charges);
        }
    });

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let form_data = LineItemForm {
            name: name_value.get(),
            description: description_value.get(),
            quantity: quantity_value.get(),
            unit_price: unit_price_value.get(),
            taxes: selected_taxes.get(),
            discounts: selected_discounts.get(),
            charges: selected_charges.get(),
        };

        let validation = form_data.validate();
        if validation.is_valid {
            grid.actions.submit_form.run(form_data);
            set_name_value.set(String::new());
            set_description_value.set(String::new());
            set_quantity_value.set(String::new());
            set_unit_price_value.set(String::new());
            set_selected_taxes.set(Vec::new());
            set_selected_discounts.set(Vec::new());
            set_selected_charges.set(Vec::new());
        }
    };

    // Helper to update form state
    let update_form = move || {
        grid.actions.update_form.run(LineItemForm {
            name: name_value.get(),
            description: description_value.get(),
            quantity: quantity_value.get(),
            unit_price: unit_price_value.get(),
            taxes: selected_taxes.get(),
            discounts: selected_discounts.get(),
            charges: selected_charges.get(),
        });
    };

    let handle_name_input = move |ev| {
        set_name_value.set(event_target_value(&ev));
        update_form();
    };
    let handle_description_input = move |ev| {
        set_description_value.set(event_target_value(&ev));
        update_form();
    };
    let handle_quantity_input = move |ev| {
        set_quantity_value.set(event_target_value(&ev));
        update_form();
    };
    let handle_unit_price_input = move |ev| {
        set_unit_price_value.set(event_target_value(&ev));
        update_form();
    };

    // let handle_taxes_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     let mut new_taxes = selected_taxes.get();
    //     if new_taxes.contains(&value) {
    //         new_taxes.retain(|x| x != &value);
    //     } else {
    //         new_taxes.push(value);
    //     }
    //     set_selected_taxes.set(new_taxes);
    //     update_form();
    // };
    //
    // let handle_discounts_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     let mut new_discounts = selected_discounts.get();
    //     if new_discounts.contains(&value) {
    //         new_discounts.retain(|x| x != &value);
    //     } else {
    //         new_discounts.push(value);
    //     }
    //     set_selected_discounts.set(new_discounts);
    //     update_form();
    // };
    //
    // let handle_charges_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     let mut new_charges = selected_charges.get();
    //     if new_charges.contains(&value) {
    //         new_charges.retain(|x| x != &value);
    //     } else {
    //         new_charges.push(value);
    //     }
    //     set_selected_charges.set(new_charges);
    //     update_form();
    // };

    view! {
        <div class="container mx-auto p-4">
            // Line Item Form Section
            <div class="bg-white p-8 rounded-xl shadow-lg w-full max-w-2xl">
                <h1 class="text-2xl font-bold text-gray-800 mb-6">
                    {move || {
                        if grid.form_state.get().is_editing {
                            "Edit Line Item"
                        } else {
                            "Add New Line Item"
                        }
                    }}
                </h1>
                <form on:submit=handle_submit>
                    <div class="space-y-6">
                        <div>
                            <label class="form-label" for="item-name">
                                "Item Name"
                            </label>
                            <input
                                class="form-input"
                                id="item-name"
                                name="item-name"
                                type="text"
                                prop:value=move || name_value.get()
                                on:input=handle_name_input
                                placeholder="e.g., Website Development"
                                required
                            />
                        </div>
                        <div>
                            <label class="form-label" for="item-description">
                                "Description"
                            </label>
                            <textarea
                                class="form-input"
                                id="item-description"
                                name="item-description"
                                prop:value=move || description_value.get()
                                on:input=handle_description_input
                                rows="2"
                                placeholder="Optional description of the service or product"
                            ></textarea>
                        </div>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
                            <div>
                                <label class="form-label" for="item-quantity">
                                    "Quantity"
                                </label>
                                <input
                                    class="form-input"
                                    id="item-quantity"
                                    name="item-quantity"
                                    type="number"
                                    step="any"
                                    min="0"
                                    prop:value=move || quantity_value.get()
                                    on:input=handle_quantity_input
                                    required
                                />
                            </div>
                            <div>
                                <label class="form-label" for="unit-price">
                                    "Unit Price"
                                </label>
                                <input
                                    class="form-input"
                                    id="unit-price"
                                    name="unit-price"
                                    type="number"
                                    step="any"
                                    min="0"
                                    prop:value=move || unit_price_value.get()
                                    on:input=handle_unit_price_input
                                    required
                                />
                            </div>
                        </div>
                        <div>
                            <LineItemCharges />
                        </div>
                        <div class="flex justify-between items-center bg-gray-100 rounded-lg p-4">
                            <h2 class="text-lg font-semibold text-gray-800">Total</h2>
                            <p class="text-3xl font-bold text-gray-800">
                                {move || format!("${:.2}", total.get())}
                            </p>
                        </div>
                    </div>
                    <div class="mt-8 pt-6 border-t border-gray-200 flex justify-end">
                        <button
                            class="bg-indigo-600 text-white font-semibold py-3 px-6 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition-colors"
                            type="submit"
                        >
                            {move || {
                                if grid.form_state.get().is_editing {
                                    "Update Item"
                                } else {
                                    "Add Item"
                                }
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Line Item Table Section
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
                                                        {format!("${:.2}", item.quantity * item.unit_price)}
                                                    </span>
                                                </div>
                                                <div class="text-sm text-gray-600 mt-2">
                                                    <p>"Qty: " {item.quantity}</p>
                                                    <p>"Unit Price: " {format!("${:.2}", item.unit_price)}</p>
                                                </div>
                                                <div class="flex justify-between items-center mt-3 text-sm text-gray-600">
                                                    <div class="flex space-x-2">
                                                        <For
                                                            each=move || item.taxes.clone()
                                                            key=|tax| tax.id.clone()
                                                            children=move |tax| {
                                                                view! { <span class="badge badge-tax">{tax.name}</span> }
                                                            }
                                                        />
                                                        <For
                                                            each=move || item.discounts.clone()
                                                            key=|discount| discount.id.clone()
                                                            children=move |discount| {
                                                                view! {
                                                                    <span class="badge badge-discount">{discount.name}</span>
                                                                }
                                                            }
                                                        />
                                                        <For
                                                            each=move || item.charges.clone()
                                                            key=|charge| charge.id.clone()
                                                            children=move |charge| {
                                                                view! {
                                                                    <span class="badge badge-charge">{charge.name}</span>
                                                                }
                                                            }
                                                        />
                                                    </div>
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
                                            "No line items found. Add your first item using the form above."
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
                                                "Item"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Qty"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Unit Price"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Taxes"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Discounts"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Charges"
                                            </th>
                                            <th
                                                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                                scope="col"
                                            >
                                                "Total"
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
                                                            {item.quantity}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            {format!("${:.2}", item.unit_price)}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            <For
                                                                each=move || item.taxes.clone()
                                                                key=|tax| tax.id.clone()
                                                                children=move |tax| {
                                                                    view! { <span class="badge badge-tax">{tax.name}</span> }
                                                                }
                                                            />
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            <For
                                                                each=move || item.discounts.clone()
                                                                key=|discount| discount.id.clone()
                                                                children=move |discount| {
                                                                    view! {
                                                                        <span class="badge badge-discount">{discount.name}</span>
                                                                    }
                                                                }
                                                            />
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                            <For
                                                                each=move || item.charges.clone()
                                                                key=|charge| charge.id.clone()
                                                                children=move |charge| {
                                                                    view! {
                                                                        <span class="badge badge-charge">{charge.name}</span>
                                                                    }
                                                                }
                                                            />
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-bold">
                                                            {format!("${:.2}", item.quantity * item.unit_price)}
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
                                                    colspan="8"
                                                >
                                                    "No line items found. Add your first item using the form above."
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
                    "Total line items: " {move || grid.item_count.get()}
                </div>
            </div>
        </div>
    }
}
