use leptos::prelude::*;
use std::sync::Arc;

use crate::components::editable_grid::{
    FormData, FormValidation, ItemData, ValidationResult, use_editable_grid, validation::validators,
};

// Assuming these structs are available from other modules
// For this component, we'll use mock data.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TaxItem {
    pub id: String,
    pub name: String,
    pub rate: f64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct DiscountItem {
    pub id: String,
    pub name: String,
    pub discount_type: String, // "FixedAmount" or "Percentage"
    pub value: f64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChargeItem {
    pub id: String,
    pub name: String,
    pub amount: f64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct CustomFieldItem {
    pub id: String,
    pub name: String,
    pub field_type: String, // "Text", "Number", etc.
    pub default_value: String,
}

// 1. Define your data structure for Line Items
#[derive(Clone, PartialEq, Debug, Default)]
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
    pub taxes: Vec<String>, // IDs of selected taxes
    pub discounts: Vec<String>,
    pub charges: Vec<String>,
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
            taxes: self.taxes.iter().map(|t| t.id.clone()).collect(),
            discounts: self.discounts.iter().map(|d| d.id.clone()).collect(),
            charges: self.charges.iter().map(|c| c.id.clone()).collect(),
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

        if let Err(_) = self.quantity.parse::<f64>() {
            results.push(ValidationResult::new().with_field_error(
                "quantity".to_string(),
                "Quantity must be a valid number".to_string(),
            ));
        }

        if let Err(_) = self.unit_price.parse::<f64>() {
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
pub fn LineItems() -> impl IntoView {
    // Mock data for other components
    let all_taxes = Arc::new(vec![
        TaxItem {
            id: "tax_1".to_string(),
            name: "VAT".to_string(),
            rate: 10.0,
        },
        TaxItem {
            id: "tax_2".to_string(),
            name: "GST".to_string(),
            rate: 5.0,
        },
    ]);
    let all_discounts = Arc::new(vec![
        DiscountItem {
            id: "discount_1".to_string(),
            name: "Early Bird".to_string(),
            discount_type: "Percentage".to_string(),
            value: 10.0,
        },
        DiscountItem {
            id: "discount_2".to_string(),
            name: "Bulk Order".to_string(),
            discount_type: "FixedAmount".to_string(),
            value: 25.0,
        },
    ]);
    let all_charges = Arc::new(vec![
        ChargeItem {
            id: "charge_1".to_string(),
            name: "Shipping".to_string(),
            amount: 15.0,
        },
        ChargeItem {
            id: "charge_2".to_string(),
            name: "Handling Fee".to_string(),
            amount: 5.0,
        },
    ]);

    let initial_line_items = vec![
        LineItem {
            id: "line_1".to_string(),
            name: "Product A".to_string(),
            description: "Software license".to_string(),
            quantity: 2.0,
            unit_price: 100.0,
            taxes: vec![all_taxes[0].clone()],
            discounts: Vec::new(),
            charges: vec![all_charges[0].clone()],
            custom_fields: Vec::new(),
        },
        LineItem {
            id: "line_2".to_string(),
            name: "Service B".to_string(),
            description: "Consulting hours".to_string(),
            quantity: 5.0,
            unit_price: 50.0,
            taxes: Vec::new(),
            discounts: vec![all_discounts[0].clone()],
            charges: Vec::new(),
            custom_fields: Vec::new(),
        },
    ];

    let grid = use_editable_grid(initial_line_items);

    // Form field signals
    let (name_value, set_name_value) = signal(String::new());
    let (description_value, set_description_value) = signal(String::new());
    let (quantity_value, set_quantity_value) = signal(String::new());
    let (unit_price_value, set_unit_price_value) = signal(String::new());
    let (selected_taxes, set_selected_taxes) = signal(Vec::<String>::new());
    let (selected_discounts, set_selected_discounts) = signal(Vec::<String>::new());
    let (selected_charges, set_selected_charges) = signal(Vec::<String>::new());

    // Computed signal for line item total
    let total = Signal::derive({
        let all_taxes = all_taxes.clone();
        let all_discounts = all_discounts.clone();
        let all_charges = all_charges.clone();
        move || {
            let form = grid.form_state.get().current_form;
            let quantity = form.quantity.parse::<f64>().unwrap_or(0.0);
            let unit_price = form.unit_price.parse::<f64>().unwrap_or(0.0);
            let mut base_total = quantity * unit_price;

            let taxes = form.taxes;
            let discounts = form.discounts;
            let charges = form.charges;

            // Apply taxes
            for tax_id in taxes {
                if let Some(tax) = all_taxes.iter().find(|t| t.id == tax_id) {
                    base_total += base_total * (tax.rate / 100.0);
                }
            }

            // Apply discounts
            for discount_id in discounts {
                if let Some(discount) = all_discounts.iter().find(|d| d.id == discount_id) {
                    match discount.discount_type.as_str() {
                        "Percentage" => base_total -= base_total * (discount.value / 100.0),
                        "FixedAmount" => base_total -= discount.value,
                        _ => {}
                    }
                }
            }

            // Apply charges
            for charge_id in charges {
                if let Some(charge) = all_charges.iter().find(|c| c.id == charge_id) {
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

    let handle_taxes_change = move |ev| {
        let value = event_target_value(&ev);
        let mut new_taxes = selected_taxes.get();
        if new_taxes.contains(&value) {
            new_taxes.retain(|x| x != &value);
        } else {
            new_taxes.push(value);
        }
        set_selected_taxes.set(new_taxes);
        update_form();
    };

    let handle_discounts_change = move |ev| {
        let value = event_target_value(&ev);
        let mut new_discounts = selected_discounts.get();
        if new_discounts.contains(&value) {
            new_discounts.retain(|x| x != &value);
        } else {
            new_discounts.push(value);
        }
        set_selected_discounts.set(new_discounts);
        update_form();
    };

    let handle_charges_change = move |ev| {
        let value = event_target_value(&ev);
        let mut new_charges = selected_charges.get();
        if new_charges.contains(&value) {
            new_charges.retain(|x| x != &value);
        } else {
            new_charges.push(value);
        }
        set_selected_charges.set(new_charges);
        update_form();
    };

    view! {
        <div class="container mx-auto p-4">
            // Line Item Form Section
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">
                    {move || {
                        let form_state = grid.form_state.get();
                        if form_state.is_editing { "Edit Line Item" } else { "Add New Line Item" }
                    }}
                </h3>

                <form on:submit=handle_submit>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div class="md:col-span-2">
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
                        <div class="md:col-span-2 lg:col-span-3">
                            <label class="form-label" for="item-description">
                                "Description"
                            </label>
                            <textarea
                                class="form-textarea"
                                id="item-description"
                                name="item-description"
                                prop:value=move || description_value.get()
                                on:input=handle_description_input
                                rows="2"
                                placeholder="Optional description of the service or product"
                            ></textarea>
                        </div>
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
                        <div class="md:col-span-2 lg:col-span-1 flex items-center justify-center bg-gray-100 rounded-md p-3">
                            <span class="text-lg font-bold text-gray-800">
                                "Total: " {move || format!("${:.2}", total.get())}
                            </span>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-6">
                        <div>
                            <label class="form-label">"Taxes"</label>
                            <div class="space-y-2">
                                <For
                                    each={
                                        let all_taxes = all_taxes.clone();
                                        move || {
                                            <Vec<TaxItem> as Clone>::clone(&all_taxes).into_iter()
                                        }
                                    }
                                    key=|tax| tax.id.clone()
                                    children=move |tax| {
                                        view! {
                                            <label class="form-checkbox-label">
                                                <input
                                                    type="checkbox"
                                                    class="form-checkbox"
                                                    on:change=handle_taxes_change
                                                    prop:value=tax.id.clone()
                                                    prop:checked=move || selected_taxes.get().contains(&tax.id)
                                                />
                                                <span class="ml-2 text-sm text-gray-700">
                                                    {format!("{} ({}%)", tax.name, tax.rate)}
                                                </span>
                                            </label>
                                        }
                                    }
                                />
                            </div>
                        </div>
                        <div>
                            <label class="form-label">"Discounts"</label>
                            <div class="space-y-2">
                                <For
                                    each={
                                        let all_discounts = all_discounts.clone();
                                        move || {
                                            <Vec<DiscountItem> as Clone>::clone(&all_discounts)
                                                .into_iter()
                                        }
                                    }
                                    key=|discount| discount.id.clone()
                                    children=move |discount| {
                                        view! {
                                            <label class="form-checkbox-label">
                                                <input
                                                    type="checkbox"
                                                    class="form-checkbox"
                                                    on:change=handle_discounts_change
                                                    prop:value=discount.id.clone()
                                                    prop:checked=move || {
                                                        selected_discounts.get().contains(&discount.id)
                                                    }
                                                />
                                                <span class="ml-2 text-sm text-gray-700">
                                                    {format!(
                                                        "{} ({} {})",
                                                        discount.name,
                                                        discount.value,
                                                        discount.discount_type,
                                                    )}
                                                </span>
                                            </label>
                                        }
                                    }
                                />
                            </div>
                        </div>
                        <div>
                            <label class="form-label">"Other Charges"</label>
                            <div class="space-y-2">
                                <For
                                    each={
                                        let all_charges = all_charges.clone();
                                        move || {
                                            <Vec<ChargeItem> as Clone>::clone(&all_charges).into_iter()
                                        }
                                    }
                                    key=|charge| charge.id.clone()
                                    children=move |charge| {
                                        view! {
                                            <label class="form-checkbox-label">
                                                <input
                                                    type="checkbox"
                                                    class="form-checkbox"
                                                    on:change=handle_charges_change
                                                    prop:value=charge.id.clone()
                                                    prop:checked=move || {
                                                        selected_charges.get().contains(&charge.id)
                                                    }
                                                />
                                                <span class="ml-2 text-sm text-gray-700">
                                                    {format!("{} (${:.2})", charge.name, charge.amount)}
                                                </span>
                                            </label>
                                        }
                                    }
                                />
                            </div>
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
                                    set_quantity_value.set(String::new());
                                    set_unit_price_value.set(String::new());
                                    set_selected_taxes.set(Vec::new());
                                    set_selected_discounts.set(Vec::new());
                                    set_selected_charges.set(Vec::new());
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
                                if form_state.is_editing { "Update Item" } else { "Add Item" }
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
